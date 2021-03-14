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
use minifb::{Key, Window, WindowOptions};
use scene::WorldRenderRequest;
use worldbuilder::WorldBuilder;

use std::time::{Duration, Instant};
use std::{fs::File, io::BufReader};

fn main() {
    let mut now = Instant::now();
    println!("Constructing world from config");
    let config_file = File::open("scene_config.json").unwrap();
    let reader = BufReader::new(config_file);
    let config: Configuration = serde_json::from_reader(reader).unwrap();

    let width = config.width as usize;
    let height = (width as Real / config.aspect_ratio) as usize;

    let world = WorldBuilder::from_config(&config);
    println!("World completed. Took {}ms", now.elapsed().as_millis());

    now = Instant::now();
    println!("Rendering scene");
    let canvas = world.render(WorldRenderRequest::new(
        config.samples_per_pixel,
        config.ray_max_depth,
        config.ray_step,
        width,
        height,
    ));
    println!("Scene rendered. Took {}ms", now.elapsed().as_millis());

    now = Instant::now();
    println!("Constructing window and buffer");
    let mut window = Window::new(
        "Crayfish Render",
        width,
        height,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to open Window");

    window.limit_update_rate(Some(Duration::from_millis(16)));
    window.topmost(true);

    let window_buffer = canvas.to_u32_vec();
    window
        .update_with_buffer(&window_buffer, width, height)
        .unwrap();

    println!(
        "Window and buffer constructed. Took {}ms",
        now.elapsed().as_millis()
    );

    now = Instant::now();
    println!("Saving as image");
    image::save_buffer(
        config.output_path,
        &canvas.to_u8_vec(),
        width as u32,
        height as u32,
        image::ColorType::Rgb8,
    )
    .unwrap();
    println!("Image saved. Took {}ms", now.elapsed().as_millis());

    println!("Opening window");
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update();
    }
}
