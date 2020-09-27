use mapbox::{JumpToOptions, LatLng, Map, MapOptions, RunLoopType, Size};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("USING OTKEN {}", env::var("MAPBOX_ACCESS_TOKEN").unwrap());
    let map = Map::new(
        RunLoopType::Default,
        Some(MapOptions {
            // access_token: None,
            access_token: Some(env::var("MAPBOX_ACCESS_TOKEN").unwrap().as_str()),
            cache_path: None,
            assets_path: None,
            width: 512,
            height: 512,
            pixel_ratio: 1,
        }),
    )
    .unwrap();

    map.load_style("mapbox://styles/mapbox/streets-v11");

    let jump = JumpToOptions {
        center: LatLng(55.680191, 12.588061),
        zoom: Some(18.),
    };

    if let Some(image) = map.jump_to(&jump).set_size(Size(2012, 2020)).render() {
        image.save("image.jpg").unwrap();
    } else {
        println!("could not render image");
    }
    println!("Map done");

    Ok(())
}
