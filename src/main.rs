mod color;
mod configuration;
mod display;
mod intersections;
mod light;
mod material;
mod math;
mod matrix;
mod ray;
mod shapes;
mod transformation;
mod vec;
mod world;
mod worldbuilder;

use configuration::Configuration;
use display::Canvas;
use ray::Ray;
use vec::Vec4d;
use worldbuilder::WorldBuilder;

use std::fs::File;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    let config_file =
        File::open("C:\\Users\\User\\source\\repos\\rust\\crayfish\\src\\scene_config.json")
            .unwrap();
    let reader = BufReader::new(config_file);
    let config: Configuration = serde_json::from_reader(reader).unwrap();

    let mut now = Instant::now();
    let mut canvas = Canvas::new(config.width as usize, config.height as usize);
    let mut file = match File::create(config.output_path.clone()) {
        Ok(file) => file,
        Err(err) => panic!("Error creating file: {}", err),
    };

    let wall_z = 10.0;
    let wall_size = 7.0;
    let half = wall_size / 2.0;
    let pixel_width_size = wall_size / canvas.width as f64;
    let pixel_height_size = wall_size / canvas.height as f64;

    let world = WorldBuilder::from_config(&config);
    let ray_origin = Vec4d::new_point(0.0, 0.0, -5.0);

    println!("Setup of scene took {}ms", now.elapsed().as_millis());

    now = Instant::now();
    for y in 0..canvas.height {
        let world_y = half - pixel_height_size * y as f64;
        for x in 0..canvas.width {
            let world_x = -half + pixel_width_size * x as f64;
            let position = Vec4d::new_point(world_x, world_y, wall_z);

            let r = Ray::new(ray_origin, (position - ray_origin).as_normalized());
            canvas.set_pixel(x, y, &world.color_at(&r));
        }
    }
    println!("Ray tracing took {}ms", now.elapsed().as_millis());

    now = Instant::now();
    let canvas_color_u8 = canvas.to_u8_vec();
    println!("Converting to u8 vec took {}ms", now.elapsed().as_millis());

    now = Instant::now();
    png_encode_mini::write_rgba_from_u8(
        &mut file,
        &canvas_color_u8,
        canvas.width as u32,
        canvas.height as u32,
    )
    .unwrap();
    println!("Saving as png took {}ms", now.elapsed().as_millis());
}
