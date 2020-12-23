use crossbeam_channel::{bounded, select, Sender};
use futures::channel::oneshot::{channel, Canceled, Sender as FutureSender};
use image::RgbaImage;
use mapbox::{JumpToOptions, LatLng, Map, MapOptions, PixelRatio, RunLoopType, Size};
use mobc::{async_trait, Manager};
use std::thread::JoinHandle;

use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("canceled")]
    Channel(#[from] Canceled),
    #[error("unknown error")]
    Unknown,
}

pub struct MapboxConnection {
    thread: Option<JoinHandle<()>>,
    kill: Sender<()>,
    sender: Sender<(Request, FutureSender<Result<RgbaImage, Error>>)>,
}

#[derive(Clone, Default, Debug)]
pub struct Request {
    pub size: Size,
    pub center: LatLng,
    pub zoom: Option<f64>,
    pub style: Option<String>,
}

struct MapState {
    style: String,
    size: Size,
    zoom: f32,
    center: LatLng,
}

impl MapboxConnection {
    fn new(opts: MapboxManagerOptions) -> Result<MapboxConnection, Error> {
        let (kill_sx, kill_rx) = bounded(1);
        let (sx, work_chan) = bounded::<(Request, FutureSender<Result<RgbaImage, Error>>)>(1);
        let thread = std::thread::spawn(move || {
            let map = Map::new(
                RunLoopType::New,
                Some(MapOptions {
                    size: opts.size,
                    pixel_ratio: opts.pixel_ratio,
                    access_token: opts.access_token.as_ref().map(|m| m.as_str()),
                    cache_path: opts.cache_path.as_ref().map(|m| m.as_str()),
                    assets_path: opts.assets_path.as_ref().map(|m| m.as_str()),
                    render_retries: 10,
                }),
            )
            .unwrap();

            let default_style = opts
                .style
                .as_ref()
                .map(|m| m.to_owned())
                .unwrap_or_else(|| "mapbox://styles/mapbox/streets-v11".to_owned());

            let default_zoom = opts.zoom.unwrap_or(5.);

            let mut state = MapState {
                style: default_style,
                size: opts.size,
                zoom: default_zoom,
                center: LatLng(0., 0.),
            };

            map.load_style(&state.style);

            loop {
                let (req, done) = select! {
                    recv(kill_rx) -> _ => {
                        break
                    },
                    recv(work_chan) -> req => req.expect("work chan closed")
                };

                // if let Some(style) = req.style {
                //     if state.style != style {
                //         map.load_style(&style);
                //         state.style = style;
                //     }
                // }
                if state.size != req.size && req.size.is_valid() {
                    map.set_size(req.size);
                    state.size = req.size;
                }

                map.jump_to(&JumpToOptions {
                    center: req.center,
                    zoom: req.zoom,
                });

                let image = match map.render() {
                    Some(s) => Ok(s),
                    None => Err(Error::Unknown),
                };

                done.send(image).ok();
            }
        });
        Ok(MapboxConnection {
            thread: Some(thread),
            kill: kill_sx,
            sender: sx,
        })
    }

    fn close(&mut self) {
        self.kill.send(()).ok();
        let thread = self.thread.take().unwrap();
        thread.join().ok();
    }
}

impl MapboxConnection {
    pub async fn render(&self, req: Request) -> Result<RgbaImage, Error> {
        let (sx, rx) = channel();

        let sender = self.sender.clone();
        tokio::task::spawn_blocking(move || match sender.send((req, sx)) {
            Ok(_) => {}
            Err(_) => {
                panic!("sender busy")
            }
        });

        let ret = rx.await?;

        ret
    }
}

impl Drop for MapboxConnection {
    fn drop(&mut self) {
        self.close();
    }
}
#[derive(Clone, Default, Debug)]
pub struct MapboxManagerOptions {
    pub size: Size,
    pub pixel_ratio: PixelRatio,
    pub access_token: Option<String>,
    pub cache_path: Option<String>,
    pub assets_path: Option<String>,
    pub style: Option<String>,
    pub zoom: Option<f32>,
    // pub custom: Option<Arc<dyn Fn(&Map) + Send + Sync>>,
}

pub struct MapboxManager {
    options: MapboxManagerOptions,
}

impl MapboxManager {
    pub fn new(options: MapboxManagerOptions) -> MapboxManager {
        MapboxManager { options }
    }
}

#[async_trait]
impl Manager for MapboxManager {
    type Connection = MapboxConnection;
    type Error = Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        MapboxConnection::new(self.options.clone())
    }

    async fn check(&self, conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
        Ok(conn)
    }
}
