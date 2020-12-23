use mapbox::{JumpToOptions, LatLng, Map, MapOptions, RunLoopType, Size};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let map = Map::new(
        RunLoopType::Default,
        Some(MapOptions {
            access_token: Some(env::var("MAPBOX_ACCESS_TOKEN").unwrap().as_str()),
            ..Default::default()
        }),
    )
    .unwrap();

    map.load_style("mapbox://styles/mapbox/streets-v11");

    let jump = JumpToOptions {
        center: LatLng(55.680191, 12.588061),
        zoom: Some(18.),
    };

    if let Some(image) = map.jump_to(&jump).set_size(Size(1280, 800)).render() {
        image.save("image.jpg").unwrap();
    } else {
        println!("could not render image");
    }
    println!("Map done");

    Ok(())
}
