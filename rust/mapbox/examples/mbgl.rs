use mapbox::{JumpToOptions, LatLng, Map, RunLoopType, Size};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let map = Map::new(RunLoopType::Default, None).unwrap();

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
