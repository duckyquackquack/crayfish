use crate::intersections::IntersectionPoint;
use crate::material::Material;
use crate::matrix::Matrix4d;
use crate::ray::Ray;
use crate::vec::Vec4d;

pub trait Shape {
    fn set_transform(&mut self, transform: Matrix4d);
    fn set_material(&mut self, material: Material);
    fn get_material(&self) -> &Material;
    fn normal_at(&self, world_point: &Vec4d) -> Vec4d;
    fn intersections(&self, ray: &Ray) -> Vec<IntersectionPoint>;
    fn hit<'a>(
        &self,
        intersection_points: &'a [IntersectionPoint],
    ) -> Option<IntersectionPoint<'a>>;
}

pub struct Sphere {
    transform: Matrix4d,
    material: Material,
}

impl Shape for Sphere {
    fn set_transform(&mut self, transform: Matrix4d) {
        self.transform = transform;
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn get_material(&self) -> &Material {
        &self.material
    }

    fn normal_at(&self, world_point: &Vec4d) -> Vec4d {
        let inverse_transform = self.transform.inverse();

        let object_point = inverse_transform * world_point;
        let object_normal = object_point - Vec4d::new_point(0.0, 0.0, 0.0);
        let world_normal = inverse_transform.transpose() * object_normal;

        world_normal.as_normalized()
    }

    fn intersections(&self, ray: &Ray) -> Vec<IntersectionPoint> {
        let transformed_ray = ray * self.transform.inverse();

        let diff = transformed_ray.origin - Vec4d::new_point(0.0, 0.0, 0.0);

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
            transform: Matrix4d::identity(),
            material: Material::default(),
        }
    }
}

#[cfg(test)]
mod sphere_tests {
    use std::vec;

    use super::Shape;
    use super::Sphere;
    use crate::intersections::IntersectionPoint;
    use crate::ray::Ray;
    use crate::transformation::TransformBuilder;
    use crate::vec::Vec4d;

    #[test]
    fn returns_two_intersections_when_hit_is_not_at_tangent() {
        let sphere = Sphere::new();
        let ray = Ray::new(
            Vec4d::new_point(0.0, 0.0, -5.0),
            Vec4d::new_vector(0.0, 0.0, 1.0),
        );

        let intersection_points = sphere.intersections(&ray);

        assert_eq!(2, intersection_points.len());
        assert_eq!(4.0, intersection_points[0].t);
        assert_eq!(6.0, intersection_points[1].t);
    }

    #[test]
    fn returns_duplicate_intersection_when_hit_is_at_tangent() {
        let sphere = Sphere::new();
        let ray = Ray::new(
            Vec4d::new_point(0.0, 1.0, -5.0),
            Vec4d::new_vector(0.0, 0.0, 1.0),
        );

        let intersection_points = sphere.intersections(&ray);

        assert_eq!(2, intersection_points.len());
        assert_eq!(5.0, intersection_points[0].t);
        assert_eq!(5.0, intersection_points[1].t);
    }

    #[test]
    fn returns_no_intesections_when_ray_misses_sphere() {
        let sphere = Sphere::new();
        let ray = Ray::new(
            Vec4d::new_point(0.0, 10.0, -5.0),
            Vec4d::new_vector(0.0, 0.0, 1.0),
        );

        let intersection_points = sphere.intersections(&ray);

        assert_eq!(0, intersection_points.len());
    }

    #[test]
    fn returns_alternating_intersections_when_ray_inside_sphere() {
        let sphere = Sphere::new();
        let ray = Ray::new(
            Vec4d::new_point(0.0, 0.0, 0.0),
            Vec4d::new_vector(0.0, 0.0, 1.0),
        );

        let intersection_points = sphere.intersections(&ray);

        assert_eq!(2, intersection_points.len());
        assert_eq!(-1.0, intersection_points[0].t);
        assert_eq!(1.0, intersection_points[1].t);
    }

    #[test]
    fn returns_two_negative_intersections_when_ray_behind_sphere() {
        let sphere = Sphere::new();
        let ray = Ray::new(
            Vec4d::new_point(0.0, 0.0, 5.0),
            Vec4d::new_vector(0.0, 0.0, 1.0),
        );

        let intersection_points = sphere.intersections(&ray);

        assert_eq!(2, intersection_points.len());
        assert_eq!(-6.0, intersection_points[0].t);
        assert_eq!(-4.0, intersection_points[1].t);
    }

    #[test]
    fn smallest_hit_when_all_intersections_positive() {
        let sphere = Sphere::new();
        let intersections = vec![
            IntersectionPoint::new(&sphere, 1.0),
            IntersectionPoint::new(&sphere, 2.0),
        ];

        let result = match sphere.hit(&intersections) {
            Some(hit) => hit,
            None => IntersectionPoint::new(&sphere, 0.0),
        };
        let expected_result = IntersectionPoint::new(&sphere, 1.0);

        assert_eq!(expected_result.t, result.t);
    }

    #[test]
    fn smallest_positive_hit_when_some_intersections_negative() {
        let sphere = Sphere::new();
        let intersections = vec![
            IntersectionPoint::new(&sphere, -1.0),
            IntersectionPoint::new(&sphere, 1.0),
        ];

        let result = match sphere.hit(&intersections) {
            Some(hit) => hit,
            None => IntersectionPoint::new(&sphere, 0.0),
        };
        let expected_result = IntersectionPoint::new(&sphere, 1.0);

        assert_eq!(expected_result.t, result.t);
    }

    #[test]
    fn no_hit_when_all_intersections_negative() {
        let sphere = Sphere::new();
        let intersections = vec![
            IntersectionPoint::new(&sphere, -1.0),
            IntersectionPoint::new(&sphere, -2.0),
        ];

        let result = sphere.hit(&intersections);

        assert!(result.is_none());
    }

    #[test]
    fn hit_is_smallest_non_negative_intersection() {
        let sphere = Sphere::new();
        let intersections = vec![
            IntersectionPoint::new(&sphere, 5.0),
            IntersectionPoint::new(&sphere, 7.0),
            IntersectionPoint::new(&sphere, -3.0),
            IntersectionPoint::new(&sphere, 2.0),
        ];

        let result = match sphere.hit(&intersections) {
            Some(hit) => hit,
            None => IntersectionPoint::new(&sphere, 0.0),
        };
        let expected_result = IntersectionPoint::new(&sphere, 2.0);

        assert_eq!(expected_result.t, result.t);
    }

    #[test]
    fn can_intersect_a_scaled_sphere() {
        let mut sphere = Sphere::new();
        sphere.set_transform(TransformBuilder::new().add_scale(2.0, 2.0, 2.0).build());

        let ray = Ray::new(
            Vec4d::new_point(0.0, 0.0, -5.0),
            Vec4d::new_vector(0.0, 0.0, 1.0),
        );

        let result = sphere.intersections(&ray);

        assert_eq!(2, result.len());
        assert_eq!(3.0, result[0].t);
        assert_eq!(7.0, result[1].t);
    }

    #[test]
    fn can_intersect_a_translated_sphere() {
        let mut sphere = Sphere::new();
        sphere.set_transform(
            TransformBuilder::new()
                .add_translation(5.0, 0.0, 0.0)
                .build(),
        );

        let ray = Ray::new(
            Vec4d::new_point(0.0, 0.0, -5.0),
            Vec4d::new_vector(0.0, 0.0, 1.0),
        );

        let result = sphere.intersections(&ray);

        assert_eq!(0, result.len());
    }

    #[test]
    fn can_get_normal_at_point_on_each_axis() {
        let sphere = Sphere::new();
        let x_axis_result = sphere.normal_at(&Vec4d::new_point(1.0, 0.0, 0.0));
        let y_axis_result = sphere.normal_at(&Vec4d::new_point(0.0, 1.0, 0.0));
        let z_axis_result = sphere.normal_at(&Vec4d::new_point(0.0, 0.0, 1.0));

        assert_eq!(x_axis_result, Vec4d::new_vector(1.0, 0.0, 0.0));
        assert_eq!(y_axis_result, Vec4d::new_vector(0.0, 1.0, 0.0));
        assert_eq!(z_axis_result, Vec4d::new_vector(0.0, 0.0, 1.0));
    }

    #[test]
    fn can_get_normal_on_nonaxial_point() {
        let sphere = Sphere::new();

        let value = f64::sqrt(3.0) / 3.0;
        let result = sphere.normal_at(&Vec4d::new_point(value, value, value));

        assert_eq!(result, Vec4d::new_vector(value, value, value));
    }

    #[test]
    fn normals_at_any_point_are_unit_vectors() {
        let sphere = Sphere::new();
        let x_axis_result = sphere.normal_at(&Vec4d::new_point(1.0, 0.0, 0.0));

        assert_eq!(x_axis_result, x_axis_result.as_normalized());
    }

    #[test]
    fn computes_normals_on_a_translated_sphere() {
        let mut sphere = Sphere::new();
        sphere.set_transform(
            TransformBuilder::new()
                .add_translation(0.0, 1.0, 0.0)
                .build(),
        );
        let result = sphere.normal_at(&Vec4d::new_point(0.0, 1.70711, -0.70711));

        assert_eq!(
            result,
            Vec4d::new_vector(0.0, 0.7071067811865475, -0.7071067811865476)
        );
    }

    #[test]
    fn computes_normals_on_a_scaled_and_rotated_sphere() {
        let mut sphere = Sphere::new();
        sphere.set_transform(
            TransformBuilder::new()
                .add_z_rotation(std::f64::consts::PI / 5.0)
                .add_scale(1.0, 0.5, 1.0)
                .build(),
        );
        let value = f64::sqrt(2.0) / 2.0;
        let result = sphere.normal_at(&Vec4d::new_point(0.0, value, -value));

        assert_eq!(
            result,
            Vec4d::new_vector(0.0, 0.9701425001453319, -0.24253562503633294)
        );
    }
}
