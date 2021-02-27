use crate::color::Color;
use crate::intersections::{IntersectionComputation, IntersectionPoint};
use crate::light::PointLight;
use crate::ray::Ray;
use crate::shapes::Shape;

use std::rc::Rc;

pub struct World {
    objects: Vec<Rc<dyn Shape>>,
    light: PointLight,
}

impl World {
    pub fn new() -> World {
        World {
            objects: Vec::new(),
            light: PointLight::default(),
        }
    }

    pub fn set_light(&mut self, light: PointLight) {
        self.light = light;
    }

    pub fn add_object(&mut self, object: Rc<dyn Shape>) {
        self.objects.push(object);
    }

    fn intersect(&self, ray: &Ray) -> Vec<IntersectionPoint> {
        let mut intersections = Vec::new();

        for object in self.objects.iter() {
            intersections.append(&mut object.intersections(ray));
        }

        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        intersections
    }

    fn shade_hit(&self, intersection: &IntersectionComputation) -> Color {
        self.light.shade(
            intersection.object.get_material(),
            &intersection.point,
            &intersection.to_eye,
            &intersection.normal,
        )
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        let intersections = self.intersect(ray);

        let non_negative_intersection_points: Vec<IntersectionPoint> = intersections
            .iter()
            .filter(|p| p.t > 0.0)
            .map(|p| *p)
            .collect();

        if non_negative_intersection_points.is_empty() {
            return Color::new(52.0 / 255.0, 198.0 / 255.0, 235.0 / 255.0);
        }

        self.shade_hit(&non_negative_intersection_points[0].prepare_computation(ray))
    }
}
