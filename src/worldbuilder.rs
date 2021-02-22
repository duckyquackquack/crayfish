use crate::color::Color;
use crate::configuration::Configuration;
use crate::light::PointLight;
use crate::material::Material;
use crate::shapes::{Shape, Sphere};
use crate::transformation::TransformBuilder;
use crate::vec::Vec4d;
use crate::world::World;

use std::rc::Rc;

pub struct WorldBuilder;

impl WorldBuilder {
    pub fn from_config(config: &Configuration) -> World {
        let mut world = World::new();

        for shape in config.shapes.iter() {
            // TODO - only supports spheres for now but will need to switch/case on the shape.type eventually
            let mut shape_instance = Sphere::new();

            let translation = &shape.transform.translation;
            let scale = &shape.transform.scale;
            let rotation = &shape.transform.rotation;

            let transform_instance = TransformBuilder::new()
                .add_translation(translation[0], translation[1], translation[2])
                .add_scale(scale[0], scale[1], scale[2])
                .add_x_rotation(rotation[0])
                .add_y_rotation(rotation[1])
                .add_z_rotation(rotation[2])
                .build();

            let material = &shape.material;
            let material_instance = Material::new(
                Color::new(material.color[0], material.color[1], material.color[2]),
                material.ambient,
                material.diffuse,
                material.specular,
                material.shininess,
            );

            shape_instance.set_transform(transform_instance);
            shape_instance.set_material(material_instance);

            let shape_ref = Rc::new(shape_instance);
            world.add_object(shape_ref.clone());
        }

        let light_config = &config.light;
        let light_position = Vec4d::new_point(
            light_config.position[0],
            light_config.position[1],
            light_config.position[2],
        );
        let light_intensity = Color::new(
            light_config.intensity[0],
            light_config.intensity[1],
            light_config.intensity[2],
        );
        let light = PointLight::new(light_intensity, light_position);
        world.set_light(light);

        world
    }
}
