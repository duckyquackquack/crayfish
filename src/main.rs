mod color;
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

use color::Color;
use display::Canvas;
use light::PointLight;
use material::Material;
use ray::Ray;
use shapes::Sphere;
use transformation::TransformBuilder;
use vec::Vec4d;

use std::fs::File;
use std::time::Instant;

fn main() {
    let mut now = Instant::now();
    let mut canvas = Canvas::new(1000, 1000);
    let mut file =
        match File::create("C:\\Users\\User\\Pictures\\crayfish_renders\\big_circle_lad.png") {
            Ok(file) => file,
            Err(err) => panic!("Error creating file: {}", err),
        };

    let wall_z = 10.0;
    let wall_size = 7.0;
    let half = wall_size / 2.0;
    let pixel_width_size = wall_size / canvas.width as f64;
    let pixel_height_size = wall_size / canvas.height as f64;

    let red = Color::new(1.0, 0.0, 0.0);
    let mut sphere = Sphere::new(0);
    let transform = TransformBuilder::new()
        .add_translation(1.0, 0.5, 5.0)
        .add_scale(1.0, 1.0, 1.0)
        .add_x_rotation(0.0)
        .add_y_rotation(0.0)
        .add_z_rotation(0.0)
        .build();
    let mut material = Material::default();
    material.color = red;
    sphere.set_transform(transform);
    sphere.set_material(material);

    let light_color = Color::new(1.0, 1.0, 1.0);
    let light_position = Vec4d::new_point(-10.0, -10.0, -10.0);
    let light = PointLight::new(light_color, light_position);

    let ray_origin = Vec4d::new_point(0.0, 0.0, -5.0);

    println!("Setup of scene took {}ms", now.elapsed().as_millis());

    now = Instant::now();
    for y in 0..canvas.height {
        let world_y = half - pixel_height_size * y as f64;
        for x in 0..canvas.width {
            let world_x = -half + pixel_width_size * x as f64;
            let position = Vec4d::new_point(world_x, world_y, wall_z);

            let r = Ray::new(ray_origin, (position - ray_origin).as_normalized());
            let intersections = sphere.intersections(&r);
            let hit = sphere.hit(&intersections);

            if hit.is_some() {
                let hit_data = hit.unwrap();
                let point = r.position_at(hit_data.t);
                let normal = sphere.normal_at(&point);
                let eye = -r.direction;

                let pixel = light.shade(&sphere.material, &point, &eye, &normal);

                canvas.set_pixel(x, y, &pixel);
            }
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
