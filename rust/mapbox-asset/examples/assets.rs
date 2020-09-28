use mapbox_asset::{self, MapPoolOptions};
use serde_json;
use std::env;
use tasks_assets::{cache::NullCache, AssetRequest, Assets, Options};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let assets = Assets::new(
        NullCache,
        mapbox_asset::create(MapPoolOptions {
            workers: 2,
            access_token: Some(env::var("MAPBOX_ACCESS_TOKEN").unwrap()),
            ..Default::default()
        }),
    );

    let out = assets
        .get(AssetRequest::new("/").with_args(serde_json::json!({
            "zoom": 16,
            "size": [256, 256]
        }))?)
        .await
        .unwrap();

    println!("RET {:?}", out.node());

    Ok(())
}
