use mapbox::{LatLng, PixelRatio, Size};
use mobc::Pool;
use mobc_mapbox::*;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = MapboxManager::new(MapboxManagerOptions {
        access_token: Some(std::env::var("MAPBOX_ACCESS_TOKEN")?),
        cache_path: Some("./test.sqlite".to_owned()),
        size: Size(512, 512),
        pixel_ratio: PixelRatio::Retina,
        ..Default::default()
    });

    let pool = Pool::builder().max_open(4).build(manager);
    let num: usize = 50;
    let (tx, mut rx) = tokio::sync::mpsc::channel::<()>(16);

    let now = Instant::now();
    for n in 0..num {
        let pool = pool.clone();
        let tx = tx.clone();
        tokio::spawn(async move {
            let conn = pool.get().await.unwrap();
            let image = conn
                .render(Request {
                    center: LatLng(55.680191, 12.588061),
                    zoom: Some(18.),
                    size: Size(800, 600),
                    ..Default::default()
                })
                .await
                .unwrap();
            image.save(format!("image-{}.jpg", n));
            tx.send(()).await.unwrap();
        });
    }

    for _ in 0..num {
        rx.recv().await.unwrap();
    }

    println!("cost: {:?}", now.elapsed());

    Ok(())
}
