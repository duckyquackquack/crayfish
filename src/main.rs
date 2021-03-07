mod camera;
mod configuration;
mod defs;
mod display;
mod material;
mod math;
mod records;
mod scene;
mod shapes;
mod worldbuilder;

use configuration::Configuration;
use defs::Real;
use scene::WorldRenderRequest;
use worldbuilder::WorldBuilder;

use std::{fs::File, io::BufReader};

fn main() {
    let config_file =
        File::open("C:\\Users\\User\\source\\repos\\rust\\crayfish\\src\\scene_config.json")
            .unwrap();
    let reader = BufReader::new(config_file);
    let config: Configuration = serde_json::from_reader(reader).unwrap();

    let world = WorldBuilder::from_config(&config);

    let width = config.width as usize;
    let height = (width as Real / config.aspect_ratio) as usize;

    let canvas = world.render(WorldRenderRequest::new(
        config.samples_per_pixel,
        config.ray_max_depth,
        config.ray_step,
        width,
        height,
    ));

    image::save_buffer(
        config.output_path,
        &canvas.to_u8_vec(),
        width as u32,
        height as u32,
        image::ColorType::Rgb8,
    )
    .unwrap();
}
