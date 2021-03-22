use crate::camera::Camera;
use crate::configuration::Configuration;
use crate::defs::Real;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::math::{Point3, Vector3};
use crate::scene::World;
use crate::shapes::Sphere;

use std::rc::Rc;

pub struct WorldBuilder;

impl WorldBuilder {
    pub fn from_config(config: &Configuration) -> World {
        let camera = create_camera(config);
        let mut world = World::new(camera);

        for shape in config.shapes.iter() {
            match &shape.type_field[..] {
                "sphere" => world.add_shape(Rc::new(create_sphere(shape))),
                _ => panic!(format!("Unsupported shape type: {}", shape.type_field)),
            }
        }

        world
    }
}

fn create_camera(config: &crate::configuration::Configuration) -> Camera {
    let camera = &config.camera;

    let origin = Point3::new(camera.position[0], camera.position[1], camera.position[2]);
    let look_at = Point3::new(camera.look_at[0], camera.look_at[1], camera.look_at[2]);
    let up = Vector3::new(camera.up[0], camera.up[1], camera.up[2]);

    let focus_distance = (origin - look_at).magnitude();

    Camera::new(
        origin,
        look_at,
        up,
        config.aspect_ratio,
        camera.fov_deg,
        focus_distance,
        camera.aperture,
    )
}

fn create_sphere(shape: &crate::configuration::Shape) -> Sphere {
    let radius: Real = shape.transform.size[0];
    let position = Point3::new(
        shape.transform.position[0],
        shape.transform.position[1],
        shape.transform.position[2],
    );

    match &shape.material.type_field[..] {
        "lambertian" => Sphere::new(
            position,
            radius,
            Rc::new(create_lambertian_material(&shape.material)),
        ),
        "metal" => Sphere::new(
            position,
            radius,
            Rc::new(create_metal_material(&shape.material)),
        ),
        "dielectric" => Sphere::new(
            position,
            radius,
            Rc::new(create_dielectric_material(&shape.material)),
        ),
        _ => panic!(format!(
            "Unsupported material type: {}",
            shape.material.type_field
        )),
    }
}

fn create_dielectric_material(material: &crate::configuration::Material) -> Dielectric {
    Dielectric::new(material.refraction_index.unwrap())
}

fn create_metal_material(material: &crate::configuration::Material) -> Metal {
    let diffuse = material.diffuse.as_ref().unwrap();
    Metal::new(
        Vector3::new(diffuse[0], diffuse[1], diffuse[2]),
        material.fuzz.unwrap_or(0.0),
    )
}

fn create_lambertian_material(material: &crate::configuration::Material) -> Lambertian {
    let diffuse = material.diffuse.as_ref().unwrap();

    Lambertian::new(Vector3::new(diffuse[0], diffuse[1], diffuse[2]))
}
