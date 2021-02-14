mod display;
mod intersections;
mod math;
mod matrix;
mod ray;
mod shapes;
mod transformation;
mod vec;

use display::{Canvas, Color};
use ray::Ray;
use shapes::Sphere;
use transformation::TransformBuilder;
use vec::Vec4d;

use std::fs::File;

fn main() {
    let mut canvas = Canvas::new(100, 100);
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

    let mut sphere = Sphere::new(0);
    let transform = TransformBuilder::new()
        .add_translation(0.0, 0.0, 0.0)
        .add_scale(1.0, 1.0, 1.0)
        .add_x_rotation(0.0)
        .add_y_rotation(0.0)
        .add_z_rotation(0.0)
        .build();
    sphere.set_transform(transform);

    let red = Color::new(255, 0, 0);
    let ray_origin = Vec4d::new_point(0.0, 0.0, -5.0);

    for y in 0..canvas.height {
        let world_y = half - pixel_height_size * y as f64;
        for x in 0..canvas.width {
            let world_x = -half + pixel_width_size * x as f64;
            let position = Vec4d::new_point(world_x, world_y, wall_z);

            let r = Ray::new(ray_origin, (position - ray_origin).as_normalized());
            let intersections = sphere.intersections(&r);
            let hit = sphere.hit(&intersections);

            if hit.is_some() {
                canvas.set_pixel(x, y, &red);
            }
        }
    }

    let canvas_color_u8 = canvas.to_u8_vec();

    png_encode_mini::write_rgba_from_u8(
        &mut file,
        &canvas_color_u8,
        canvas.width as u32,
        canvas.height as u32,
    )
    .unwrap();
}
