use crate::intersections::IntersectionPoint;
use crate::material::Material;
use crate::ray::Ray;

use nalgebra::{Matrix4, Vector4};

pub trait Shape {
    fn set_transform(&mut self, transform: Matrix4<f64>);
    fn set_material(&mut self, material: Material);
    fn get_material(&self) -> &Material;
    fn normal_at(&self, world_point: &Vector4<f64>) -> Vector4<f64>;
    fn intersections(&self, ray: &Ray) -> Vec<IntersectionPoint>;
    fn hit<'a>(
        &self,
        intersection_points: &'a [IntersectionPoint],
    ) -> Option<IntersectionPoint<'a>>;
}

pub struct Sphere {
    transform: Matrix4<f64>,
    material: Material,
}

impl Shape for Sphere {
    fn set_transform(&mut self, transform: Matrix4<f64>) {
        self.transform = transform;
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn get_material(&self) -> &Material {
        &self.material
    }

    fn normal_at(&self, world_point: &Vector4<f64>) -> Vector4<f64> {
        let inverse_transform = self.transform.try_inverse().unwrap();

        let object_point = inverse_transform * world_point;
        let object_normal = object_point - Vector4::new(0.0, 0.0, 0.0, 1.0);
        let mut world_normal = inverse_transform.transpose() * object_normal;
        world_normal[3] = 0.0;

        world_normal.normalize()
    }

    fn intersections(&self, ray: &Ray) -> Vec<IntersectionPoint> {
        let transformed_ray = ray * self.transform.try_inverse().unwrap();

        let diff = transformed_ray.origin - Vector4::new(0.0, 0.0, 0.0, 1.0);

        let a = transformed_ray.direction.dot(&transformed_ray.direction);
        let b = 2.0 * transformed_ray.direction.dot(&diff);
        let c = diff.dot(&diff) - 1.0;

        let discriminant = b.powf(2.0) - (4.0 * a * c);
        if discriminant < 0.0 {
            return Vec::new();
        }

        let intersection_points = vec![
            IntersectionPoint::new(self, (-b - discriminant.sqrt()) / (2.0 * a)),
            IntersectionPoint::new(self, (-b + discriminant.sqrt()) / (2.0 * a)),
        ];

        intersection_points
    }

    fn hit<'a>(
        &self,
        intersection_points: &'a [IntersectionPoint],
    ) -> Option<IntersectionPoint<'a>> {
        let mut non_negative_intersection_points: Vec<IntersectionPoint> = intersection_points
            .iter()
            .filter(|p| p.t > 0.0)
            .map(|p| *p)
            .collect();

        if non_negative_intersection_points.is_empty() {
            return None;
        }

        non_negative_intersection_points.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        Some(non_negative_intersection_points[0])
    }
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            transform: Matrix4::identity(),
            material: Material::default(),
        }
    }
}
