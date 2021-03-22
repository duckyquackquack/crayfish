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

use camera::Camera;
use configuration::Configuration;
use defs::Real;
use material::{Dielectric, Lambertian, Metal};
use math::{Color3, Point3, Vector3};
use minifb::{Key, Window, WindowOptions};
use rand::Rng;
use scene::{World, WorldRenderRequest};
use shapes::Sphere;
use worldbuilder::WorldBuilder;

use std::rc::Rc;
use std::time::{Duration, Instant};
use std::{fs::File, io::BufReader};

fn random_scene() -> World {
    let camera = Camera::new(
        Vector3::new(13.0, 2.0, 3.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        16.0 / 9.0,
        20.0,
        10.0,
        0.1,
    );

    let mut world = World::new(camera);

    let ground_material = Lambertian::new(Color3::new(0.5, 0.5, 0.5));
    let ground = Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(ground_material),
    );

    world.add_shape(Rc::new(ground));

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let rand_material_choice = rng.gen_range(0.0..1.0);
            let center = Point3::new(
                (a as Real) + 0.9 * rng.gen_range(0.0..1.0),
                0.2,
                (b as Real) + 0.9 * rng.gen_range(0.0..1.0),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                if rand_material_choice < 0.8 {
                    let diffuse = Color3::new_random(0.0, 1.0) * Color3::new_random(0.0, 1.0);
                    let material = Lambertian::new(diffuse);
                    let sphere = Sphere::new(center, 0.2, Rc::new(material));

                    world.add_shape(Rc::new(sphere));
                } else if rand_material_choice < 0.95 {
                    let diffuse = Color3::new_random(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let material = Metal::new(diffuse, fuzz);
                    let sphere = Sphere::new(center, 0.2, Rc::new(material));

                    world.add_shape(Rc::new(sphere));
                } else {
                    let material = Dielectric::new(1.5);
                    let sphere = Sphere::new(center, 0.2, Rc::new(material));

                    world.add_shape(Rc::new(sphere));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    let sphere1 = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, Rc::new(material1));
    world.add_shape(Rc::new(sphere1));

    let material2 = Lambertian::new(Color3::new(0.4, 0.2, 0.1));
    let sphere2 = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, Rc::new(material2));
    world.add_shape(Rc::new(sphere2));

    let material3 = Metal::new(Color3::new(0.7, 0.6, 0.5), 0.0);
    let sphere3 = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, Rc::new(material3));
    world.add_shape(Rc::new(sphere3));

    world
}

fn ray_tracing_in_one_weekend_scene() {
    let mut now = Instant::now();
    let world = random_scene();
    println!("World completed. Took {}ms", now.elapsed().as_millis());

    now = Instant::now();
    println!("Rendering scene");
    let width = 1200;
    let aspect_ratio = 16.0 / 9.0;
    let height = (width as Real / aspect_ratio) as usize;
    let canvas = world.render(WorldRenderRequest::new(500, 50, 1, width, height));
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
        "C:\\Users\\User\\Pictures\\crayfish_renders\\output.png",
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

fn render_from_config() {
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
        "C:\\Users\\User\\Pictures\\crayfish_renders\\output.png",
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

fn main() {
    render_from_config();
}
