use crossbeam_channel::{bounded, select, unbounded, Sender};
use image::DynamicImage;
use mapbox::{JumpToOptions, LatLng, Map, MapOptions, RunLoopType, Size};
use std::sync::Arc;
use std::thread::{JoinHandle, Thread};

struct Worker {
    thread: JoinHandle<()>,
    kill: Sender<()>,
}

pub struct Request {
    pub size: Size,
    pub center: LatLng,
    pub zoom: Option<f64>,
    pub style: Option<String>,
}

#[derive(Clone)]
pub struct MapPoolOptions {
    pub workers: i32,
    pub size: Size,
    pub pixel_ratio: i32,
    pub access_token: Option<String>,
    pub cache_path: Option<String>,
    pub assets_path: Option<String>,
    pub style: Option<String>,
    pub zoom: Option<f32>,
    pub custom: Option<Arc<dyn Fn(&Map) + Send + Sync>>,
}

impl Default for MapPoolOptions {
    fn default() -> Self {
        MapPoolOptions {
            workers: 1,
            size: Size(512, 512),
            pixel_ratio: 1,
            access_token: None,
            cache_path: None,
            assets_path: None,
            style: None,
            zoom: None,
            custom: None,
        }
    }
}

struct MapPoolInner {
    workers: Vec<Worker>,
    producer: Sender<(Request, Sender<Option<DynamicImage>>)>,
}

struct MapState {
    style: String,
    size: Size,
    zoom: f32,
    center: LatLng,
}

// fn render(map: &Map, state: &mut MapState, req: &Request, opts: &MapPoolOptions) {
//     if state.center != req.center {
//         Some(req.center)
//     } else {
//         None
//     }
//     if state.style != req.style {
//         map.load_style(&req.style);
//         state.style = req.style;
//     }

//     if state.size != req.size {
//         map.set_size(req.size);
//         state.size = req.size;
//     }

//     map.jump_to(&JumpToOptions {
//         center: req.center,
//         zoom: Some(req.zoom),
//     });

//     let image = map.render();
// }

impl MapPoolInner {
    fn new(options: MapPoolOptions) -> MapPoolInner {
        let mut workers = Vec::new();
        let (sx, rx) = unbounded::<(Request, Sender<Option<DynamicImage>>)>();
        for _ in 0..options.workers {
            let work_chan = rx.clone();
            let (kill_sx, kill_rx) = bounded(0);
            let opts = options.clone();
            workers.push(Worker {
                thread: std::thread::spawn(move || {
                    //
                    let map = Map::new(
                        RunLoopType::New,
                        Some(MapOptions {
                            width: opts.size.0,
                            height: opts.size.1,
                            pixel_ratio: opts.pixel_ratio,
                            access_token: opts.access_token.as_ref().map(|m| m.as_str()),
                            cache_path: opts.cache_path.as_ref().map(|m| m.as_str()),
                            assets_path: opts.assets_path.as_ref().map(|m| m.as_str()),
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

                        if let Some(style) = req.style {
                            if state.style != style {
                                map.load_style(&style);
                                state.style = style;
                            }
                        }
                        if state.size != req.size {
                            map.set_size(req.size);
                            state.size = req.size;
                        }

                        map.jump_to(&JumpToOptions {
                            center: req.center,
                            zoom: req.zoom,
                        });

                        let image = map.render();

                        done.send(image).ok();
                    }

                    ()
                }),
                kill: kill_sx,
            });
        }

        MapPoolInner {
            workers,
            producer: sx,
        }
    }
    fn close(&mut self) {
        self.workers.iter().for_each(|worker| {
            worker.kill.send(()).ok();
        });

        for worker in self.workers.drain(0..self.workers.len()) {
            worker.thread.join().unwrap();
        }
    }
}

impl Drop for MapPoolInner {
    fn drop(&mut self) {
        self.close();
    }
}

#[derive(Clone)]
pub struct MapPool(Arc<MapPoolInner>);

impl MapPool {
    pub fn new(options: MapPoolOptions) -> MapPool {
        MapPool(Arc::new(MapPoolInner::new(options)))
    }

    pub async fn render(&self, req: Request) -> Option<DynamicImage> {
        let producer = self.0.producer.clone();
        let ret = tokio::task::spawn_blocking(move || {
            let (sx, rx) = bounded(0);

            producer.send((req, sx)).unwrap();

            let ret = rx.recv().unwrap();

            ret
        })
        .await
        .unwrap();

        ret
    }
}
