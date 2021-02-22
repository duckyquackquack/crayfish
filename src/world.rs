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

#[cfg(test)]
mod world_tests {
    use super::World;
    use crate::color::Color;
    use crate::light::PointLight;
    use crate::material::Material;
    use crate::ray::Ray;
    use crate::shapes::*;
    use crate::transformation::TransformBuilder;
    use crate::vec::Vec4d;

    use std::rc::Rc;

    #[test]
    fn intersects_a_ray() {
        let ray = Ray::new(
            Vec4d::new_point(0.0, 0.0, -5.0),
            Vec4d::new_vector(0.0, 0.0, 1.0),
        );

        let sphere1 = Sphere::new();
        let mut sphere2 = Sphere::new();

        let t2 = TransformBuilder::new().add_scale(0.5, 0.5, 0.5).build();
        sphere2.set_transform(t2);

        let mut world = World::new();
        let sphere_1_ref = Rc::new(sphere1);
        let sphere_2_ref = Rc::new(sphere2);
        world.add_object(sphere_1_ref.clone());
        world.add_object(sphere_2_ref.clone());

        let intersections = world.intersect(&ray);

        assert_eq!(4, intersections.len());
        assert_eq!(4.0, intersections[0].t);
        assert_eq!(4.5, intersections[1].t);
        assert_eq!(5.5, intersections[2].t);
        assert_eq!(6.0, intersections[3].t);
    }

    #[test]
    fn shading_an_intersection() {
        let ray = Ray::new(
            Vec4d::new_point(0.0, 0.0, -5.0),
            Vec4d::new_vector(0.0, 0.0, 1.0),
        );

        let mut sphere1 = Sphere::new();
        let material = Material::new(Color::new(0.8, 1.0, 0.6), 0.1, 0.7, 0.2, 200.0);
        sphere1.set_material(material);

        let sphere_ref = Rc::new(sphere1);
        let mut world = World::new();
        world.add_object(sphere_ref.clone());

        let intersections = world.intersect(&ray);
        let comp = intersections[0].prepare_computation(&ray);
        let result_color = world.shade_hit(&comp);

        assert_eq!(
            result_color,
            Color::new(
                0.38066119308103435,
                0.47582649135129296,
                0.28549589481077575
            )
        );
    }

    #[test]
    fn shading_intersection_from_inside() {
        let ray = Ray::new(
            Vec4d::new_point(0.0, 0.0, 0.0),
            Vec4d::new_vector(0.0, 0.0, 1.0),
        );

        let mut sphere1 = Sphere::new();
        let t1 = TransformBuilder::new().add_scale(0.5, 0.5, 0.5).build();
        sphere1.set_transform(t1);

        let mut world = World::new();
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Vec4d::new_point(0.0, 0.25, 0.0));
        let sphere_ref = Rc::new(sphere1);
        world.add_object(sphere_ref.clone());
        world.set_light(light);

        let intersections = world.intersect(&ray);
        let comp = intersections[1].prepare_computation(&ray);
        let result_color = world.shade_hit(&comp);

        assert_eq!(
            result_color,
            Color::new(0.9049844720832575, 0.9049844720832575, 0.9049844720832575)
        );
    }
}
