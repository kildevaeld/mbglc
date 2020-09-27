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
    pub zoom: f64,
    pub style: String,
}

#[derive(Clone)]
pub struct MapPoolOptions {
    pub workers: i32,
    pub size: Size,
    pub pixel_ratio: i32,
    pub access_token: Option<String>,
    pub cache_path: Option<String>,
    pub assets_path: Option<String>,
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
        }
    }
}

struct MapPoolInner {
    workers: Vec<Worker>,
    producer: Sender<(Request, Sender<Option<DynamicImage>>)>,
}

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

                    map.load_style("mapbox://styles/mapbox/streets-v11");

                    loop {
                        let (req, done) = select! {
                            recv(kill_rx) -> _ => {
                                break
                            },
                            recv(work_chan) -> req => req.expect("work chan closed")
                        };
                        map.load_style(&req.style);
                        map.set_size(req.size);
                        map.jump_to(&JumpToOptions {
                            center: req.center,
                            zoom: Some(req.zoom),
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
