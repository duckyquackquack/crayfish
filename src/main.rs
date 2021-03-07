mod camera;
mod defs;
mod display;
mod material;
mod math;
mod records;
mod scene;
mod shapes;

use defs::Real;
use material::{Lambertian, Metal};
use math::{Color3, Point3};
use scene::World;
use shapes::Sphere;

use std::rc::Rc;

fn main() {
    let aspect_ratio: Real = 16.0 / 9.0;
    let width: usize = 400;
    let height: usize = (width as Real / aspect_ratio) as usize;

    let mut world = World::new();

    let ground = Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::new(Lambertian::new(Color3::new(0.8, 0.8, 0.0))),
    );
    let middle_sphere = Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        Rc::new(Lambertian::new(Color3::new(0.2, 0.5, 0.9))),
    );
    let left_sphere = Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Rc::new(Metal::new(Color3::new(0.8, 0.8, 0.8), 0.3)),
    );
    let right_sphere = Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Rc::new(Metal::new(Color3::new(0.8, 0.6, 0.2), 1.0)),
    );

    world.add_shape(Rc::new(middle_sphere));
    world.add_shape(Rc::new(left_sphere));
    world.add_shape(Rc::new(right_sphere));
    world.add_shape(Rc::new(ground));

    let canvas = world.render(width, height);

    image::save_buffer(
        "C:\\Users\\User\\Pictures\\crayfish_renders\\output.png",
        &canvas.to_u8_vec(),
        width as u32,
        height as u32,
        image::ColorType::Rgb8,
    )
    .unwrap();
}
