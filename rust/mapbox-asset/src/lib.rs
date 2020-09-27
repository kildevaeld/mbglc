mod pool;
use bytes::{buf::BufMutExt, BufMut, BytesMut};
use image::ImageOutputFormat;
use mapbox::{LatLng, MapOptions, Size};
use mime::IMAGE_PNG;
use tasks::{task, Rejection, Task};
use tasks_assets::{AssetRequest, AssetResponse, Error, Node};
use tasks_vinyl::{Content, File};

pub use pool::{MapPool, MapPoolOptions, Request as MapRequest};

pub fn create(
    options: MapPoolOptions,
) -> impl Task<AssetRequest, Output = AssetResponse, Error = Error> + Clone {
    let pool = MapPool::new(options);
    task!(move |req: AssetRequest| {
        let pool = pool.clone();
        async move {
            //

            let size = Size(1280, 1024);
            let center = LatLng(55.680191, 12.588061);

            let ret = pool
                .render(MapRequest {
                    size,
                    center,
                    zoom: 15.,
                    style: "mapbox://styles/mapbox/streets-v11".to_owned(),
                })
                .await;

            let image = match ret {
                Some(i) => i,
                None => {
                    return Err(Rejection::Reject(req, Some(Error::NotFound)));
                }
            };

            let name = match req.path() {
                "/" => format!("map-{}x{}-{}x{}.png", size.0, size.1, center.0, center.1),
                path => path.to_owned(),
            };

            let mut buf = BytesMut::new().writer();
            image.write_to(&mut buf, ImageOutputFormat::Png).unwrap();
            let buf = buf.into_inner();
            let len = buf.len();
            let content = Content::from(buf.freeze());
            let file = File::new(name, content, IMAGE_PNG, len as u64);

            Ok(req.reply(Node::File(file)))
        }
    })
}
