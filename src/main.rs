mod camera;
mod color;
mod configuration;
mod display;
mod intersections;
mod light;
mod material;
mod ray;
mod shapes;
mod transformation;
mod world;
mod worldbuilder;

use configuration::Configuration;
use display::Canvas;
use ray::Ray;
use worldbuilder::WorldBuilder;

use std::time::Instant;
use std::{fs::File, io::BufReader};

use nalgebra::Vector4;

fn main() {
    let config_file =
        File::open("C:\\Users\\User\\source\\repos\\rust\\crayfish\\src\\scene_config.json")
            .unwrap();
    let reader = BufReader::new(config_file);
    let config: Configuration = serde_json::from_reader(reader).unwrap();

    let mut now = Instant::now();
    let mut canvas = Canvas::new(config.width as usize, config.height as usize);

    let wall_z = 10.0;
    let wall_size = 7.0;
    let half = wall_size / 2.0;
    let pixel_width_size = wall_size / canvas.width as f64;
    let pixel_height_size = wall_size / canvas.height as f64;

    let world = WorldBuilder::from_config(&config);
    let ray_origin = Vector4::new(0.0, 0.0, -5.0, 1.0);

    println!("Setup of scene took {}ms", now.elapsed().as_millis());

    now = Instant::now();
    for y in 0..canvas.height {
        let world_y = half - pixel_height_size * y as f64;
        for x in 0..canvas.width {
            if (x + y) % 2 == 0 {
                let world_x = -half + pixel_width_size * x as f64;
                let position = Vector4::new(world_x, world_y, wall_z, 1.0);

                let r = Ray::new(ray_origin, (position - ray_origin).normalize());
                canvas.set_pixel(x, y, &world.color_at(&r));
            }
        }
    }
    println!("Ray tracing took {}ms", now.elapsed().as_millis());

    now = Instant::now();
    let canvas_color_u8 = canvas.to_u8_vec();
    println!("Converting to u8 vec took {}ms", now.elapsed().as_millis());

    now = Instant::now();
    image::save_buffer(
        config.output_path,
        &canvas_color_u8,
        config.width as u32,
        config.height as u32,
        image::ColorType::Rgb8,
    )
    .unwrap();
    println!("Saving as img took {}ms", now.elapsed().as_millis());
}
