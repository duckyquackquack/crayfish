use crate::intersections::IntersectionPoint;
use crate::matrix::Matrix4d;
use crate::ray::Ray;
use crate::vec::Vec4d;

pub struct Sphere {
    object_id: u32,
    transform: Matrix4d,
}

impl Sphere {
    pub fn new(object_id: u32) -> Sphere {
        Sphere {
            object_id,
            transform: Matrix4d::identity(),
        }
    }

    pub fn set_transform(&mut self, transform: Matrix4d) {
        self.transform = transform;
    }

    pub fn intersections(&self, ray: &Ray) -> Vec<IntersectionPoint> {
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
            IntersectionPoint::new(self.object_id, (-b - discriminant.sqrt()) / (2.0 * a)),
            IntersectionPoint::new(self.object_id, (-b + discriminant.sqrt()) / (2.0 * a)),
        ];

        intersection_points
    }

    pub fn hit(&self, intersection_points: &Vec<IntersectionPoint>) -> Option<IntersectionPoint> {
        let mut non_negative_intersection_points: Vec<IntersectionPoint> = intersection_points
            .iter()
            .filter(|p| p.t > 0.0)
            .map(|p| *p)
            .collect();

        if non_negative_intersection_points.len() == 0 {
            return None;
        }

        non_negative_intersection_points.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        Some(non_negative_intersection_points[0])
    }
}

#[cfg(test)]
mod sphere_tests {
    use std::vec;

    use super::Sphere;
    use crate::intersections::IntersectionPoint;
    use crate::ray::Ray;
    use crate::transformation::TransformBuilder;
    use crate::vec::Vec4d;

    #[test]
    fn returns_two_intersections_when_hit_is_not_at_tangent() {
        let sphere = Sphere::new(0);
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
        let sphere = Sphere::new(0);
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
        let sphere = Sphere::new(0);
        let ray = Ray::new(
            Vec4d::new_point(0.0, 10.0, -5.0),
            Vec4d::new_vector(0.0, 0.0, 1.0),
        );

        let intersection_points = sphere.intersections(&ray);

        assert_eq!(0, intersection_points.len());
    }

    #[test]
    fn returns_alternating_intersections_when_ray_inside_sphere() {
        let sphere = Sphere::new(0);
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
        let sphere = Sphere::new(0);
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
        let sphere = Sphere::new(0);
        let intersections = vec![
            IntersectionPoint::new(0, 1.0),
            IntersectionPoint::new(0, 2.0),
        ];

        let result = match sphere.hit(&intersections) {
            Some(hit) => hit,
            None => IntersectionPoint::new(999, 0.0),
        };
        let expected_result = IntersectionPoint::new(0, 1.0);

        assert_eq!(expected_result, result);
    }

    #[test]
    fn smallest_positive_hit_when_some_intersections_negative() {
        let sphere = Sphere::new(0);
        let intersections = vec![
            IntersectionPoint::new(0, -1.0),
            IntersectionPoint::new(0, 1.0),
        ];

        let result = match sphere.hit(&intersections) {
            Some(hit) => hit,
            None => IntersectionPoint::new(999, 0.0),
        };
        let expected_result = IntersectionPoint::new(0, 1.0);

        assert_eq!(expected_result, result);
    }

    #[test]
    fn no_hit_when_all_intersections_negative() {
        let sphere = Sphere::new(0);
        let intersections = vec![
            IntersectionPoint::new(0, -1.0),
            IntersectionPoint::new(0, -2.0),
        ];

        let result = sphere.hit(&intersections);

        assert_eq!(None, result);
    }

    #[test]
    fn hit_is_smallest_non_negative_intersection() {
        let sphere = Sphere::new(0);
        let intersections = vec![
            IntersectionPoint::new(0, 5.0),
            IntersectionPoint::new(0, 7.0),
            IntersectionPoint::new(0, -3.0),
            IntersectionPoint::new(0, 2.0),
        ];

        let result = match sphere.hit(&intersections) {
            Some(hit) => hit,
            None => IntersectionPoint::new(999, 0.0),
        };
        let expected_result = IntersectionPoint::new(0, 2.0);

        assert_eq!(expected_result, result);
    }

    #[test]
    fn can_intersect_a_scaled_sphere() {
        let mut sphere = Sphere::new(0);
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
        let mut sphere = Sphere::new(0);
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
}
