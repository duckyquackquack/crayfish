use crate::camera::Camera;
use crate::configuration::Configuration;
use crate::defs::Real;
use crate::material::Material;
use crate::math::{Color3, Point3, Vector3};
use crate::scene::World;
use crate::shapes::Shape;

pub struct WorldBuilder;

impl WorldBuilder {
    pub fn from_config(config: &Configuration) -> World {
        let camera = create_camera(config);
        let mut world = World::new(camera);

        for shape in config.shapes.iter() {
            match &shape.type_field[..] {
                "sphere" => world.add_shape(create_sphere(shape)),
                _ => panic!("Unsupported shape type"),
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

fn create_sphere(shape: &crate::configuration::Shape) -> Shape {
    let radius: Real = shape.transform.size[0];
    let position = Point3::new(
        shape.transform.position[0],
        shape.transform.position[1],
        shape.transform.position[2],
    );

    match &shape.material.type_field[..] {
        "lambertian" => Shape::Sphere {
            center: position,
            radius,
            material: create_lambertian_material(&shape.material),
        },
        "metal" => Shape::Sphere {
            center: position,
            radius,
            material: create_metal_material(&shape.material),
        },
        "dielectric" => Shape::Sphere {
            center: position,
            radius,
            material: create_dielectric_material(&shape.material),
        },
        _ => panic!("Unsupported material type"),
    }
}

fn create_dielectric_material(material: &crate::configuration::Material) -> Material {
    Material::Dielectric {
        refraction_index: material.refraction_index.unwrap(),
    }
}

fn create_metal_material(material: &crate::configuration::Material) -> Material {
    let diffuse = material.diffuse.as_ref().unwrap();
    Material::Metal {
        diffuse: Color3::new(diffuse[0], diffuse[1], diffuse[2]),
        fuzz: material.fuzz.unwrap_or(0.0),
    }
}

fn create_lambertian_material(material: &crate::configuration::Material) -> Material {
    let diffuse = material.diffuse.as_ref().unwrap();

    Material::Lambertian {
        diffuse: Vector3::new(diffuse[0], diffuse[1], diffuse[2]),
    }
}
