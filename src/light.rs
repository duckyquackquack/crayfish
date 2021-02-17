use crate::color::Color;
use crate::material::Material;
use crate::vec::Vec4d;

pub struct PointLight {
    intensity: Color,
    position: Vec4d,
}

impl PointLight {
    pub fn new(intensity: Color, position: Vec4d) -> PointLight {
        PointLight {
            intensity,
            position,
        }
    }

    //TODO: Not entirely convinced this is the right place for this logic
    pub fn shade(&self, material: &Material, point: &Vec4d, eye: &Vec4d, normal: &Vec4d) -> Color {
        let effective_color = material.color * self.intensity;
        let to_light = (self.position - point).as_normalized();

        let ambient = effective_color * material.ambient;
        let mut diffuse = Color::new(0.0, 0.0, 0.0);
        let mut specular = Color::new(0.0, 0.0, 0.0);

        let light_dot_normal = to_light.dot(normal);
        if light_dot_normal > 0.0 {
            diffuse = effective_color * material.diffuse * light_dot_normal;

            let reflection = -to_light.reflect_around(normal);
            let reflect_dot_eye = reflection.dot(eye);
            if reflect_dot_eye > 0.0 {
                specular =
                    self.intensity * material.specular * (reflect_dot_eye.powf(material.shininess));
            }
        }

        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod light_tests {
    use super::PointLight;
    use crate::color::Color;
    use crate::material::Material;
    use crate::vec::Vec4d;

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let material = Material::default();
        let point = Vec4d::new_point(0.0, 0.0, 0.0);

        let eye = Vec4d::new_vector(0.0, 0.0, -1.0);
        let normal = Vec4d::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Vec4d::new_point(0.0, 0.0, -10.0));

        let result = light.shade(&material, &point, &eye, &normal);
        let expected_result = Color::new(1.9, 1.9, 1.9);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_with_eye_offset_45_deg() {
        let material = Material::default();
        let point = Vec4d::new_point(0.0, 0.0, 0.0);

        let eye = Vec4d::new_vector(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0);
        let normal = Vec4d::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Vec4d::new_point(0.0, 0.0, -10.0));

        let result = light.shade(&material, &point, &eye, &normal);
        let expected_result = Color::new(1.0, 1.0, 1.0);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn lighting_with_eye_opposite_surface_and_light_offset_45_deg() {
        let material = Material::default();
        let point = Vec4d::new_point(0.0, 0.0, 0.0);

        let eye = Vec4d::new_vector(0.0, 0.0, -1.0);
        let normal = Vec4d::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(
            Color::new(1.0, 1.0, 1.0),
            Vec4d::new_point(0.0, 10.0, -10.0),
        );

        let result = light.shade(&material, &point, &eye, &normal);
        let expected_result =
            Color::new(0.7363961030678927, 0.7363961030678927, 0.7363961030678927);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection_vector() {
        let material = Material::default();
        let point = Vec4d::new_point(0.0, 0.0, 0.0);

        let eye = Vec4d::new_vector(0.0, -f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0);
        let normal = Vec4d::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(
            Color::new(1.0, 1.0, 1.0),
            Vec4d::new_point(0.0, 10.0, -10.0),
        );

        let result = light.shade(&material, &point, &eye, &normal);
        let expected_result =
            Color::new(1.6363961030678928, 1.6363961030678928, 1.6363961030678928);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        let material = Material::default();
        let point = Vec4d::new_point(0.0, 0.0, 0.0);

        let eye = Vec4d::new_vector(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0);
        let normal = Vec4d::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Vec4d::new_point(0.0, 0.0, 10.0));

        let result = light.shade(&material, &point, &eye, &normal);
        let expected_result = Color::new(0.1, 0.1, 0.1);

        assert_eq!(result, expected_result);
    }
}
