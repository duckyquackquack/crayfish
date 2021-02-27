use crate::color::Color;
use crate::material::Material;

use nalgebra::Vector4;

pub struct PointLight {
    intensity: Color,
    position: Vector4<f64>,
}

impl PointLight {
    pub fn new(intensity: Color, position: Vector4<f64>) -> PointLight {
        PointLight {
            intensity,
            position,
        }
    }

    pub fn default() -> PointLight {
        PointLight {
            intensity: Color::new(1.0, 1.0, 1.0),
            position: Vector4::new(-10.0, 10.0, -10.0, 1.0),
        }
    }

    pub fn shade(
        &self,
        material: &Material,
        point: &Vector4<f64>,
        eye: &Vector4<f64>,
        normal: &Vector4<f64>,
    ) -> Color {

        let effective_color = material.color * self.intensity;
        let to_light = (self.position - point).normalize();

        let ambient = effective_color * material.ambient;
        let mut diffuse = Color::new(0.0, 0.0, 0.0);
        let mut specular = Color::new(0.0, 0.0, 0.0);

        let light_dot_normal = to_light.dot(normal);

        if light_dot_normal > 0.0 {
            diffuse = effective_color * material.diffuse * light_dot_normal;

            let reflection_scale = 2.0 * to_light.dot(normal);
            let scaled_normal = normal * reflection_scale;
            let scaled_light_from_normal = to_light - scaled_normal;
            let reflection = -scaled_light_from_normal;
            let reflect_dot_eye = reflection.dot(eye);

            if reflect_dot_eye > 0.0 {
                specular =
                    self.intensity * material.specular * (reflect_dot_eye.powf(material.shininess));
            }
        }

        ambient + diffuse + specular
    }
}
