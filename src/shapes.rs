use crate::{intersections::IntersectionPoint, ray::Ray, vec::Vec4d};
struct Sphere {
    object_id: u32,
}

impl Sphere {
    pub fn new(object_id: u32) -> Sphere {
        Sphere { object_id }
    }

    fn intersects(&self, ray: &Ray) -> Vec<IntersectionPoint> {
        let diff = ray.origin - Vec4d::new_point(0.0, 0.0, 0.0);

        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&diff);
        let c = diff.dot(&diff) - 1.0;

        let discriminant = b.powf(2.0) - (4.0 * a * c);
        if discriminant < 0.0 {
            return Vec::new();
        }

        let mut intersection_points = Vec::new();
        intersection_points.push(IntersectionPoint::new(
            self.object_id,
            (-b - discriminant.sqrt()) / (2.0 * a),
        ));
        intersection_points.push(IntersectionPoint::new(
            self.object_id,
            (-b + discriminant.sqrt()) / (2.0 * a),
        ));

        intersection_points
    }

    pub fn hit(&self, ray: &Ray) -> Option<IntersectionPoint> {
        let intersection_points = self.intersects(ray);

        if intersection_points.len() == 0 {
            return None;
        } else {
            // filter out negative numbers
            // sort by lowest t value
            // return first in sorted list
            Some(IntersectionPoint::new(0, 0.0))
        }
    }
}

#[cfg(test)]
mod sphere_tests {
    use super::Sphere;
    use crate::ray::Ray;
    use crate::vec::Vec4d;

    #[test]
    fn returns_two_intersections_when_hit_is_not_at_tangent() {
        let sphere = Sphere::new(0);
        let ray = Ray::new(
            Vec4d::new_point(0.0, 0.0, -5.0),
            Vec4d::new_vector(0.0, 0.0, 1.0),
        );

        let intersection_points = sphere.intersects(&ray);

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

        let intersection_points = sphere.intersects(&ray);

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

        let intersection_points = sphere.intersects(&ray);

        assert_eq!(0, intersection_points.len());
    }

    #[test]
    fn returns_alternating_intersections_when_ray_inside_sphere() {
        let sphere = Sphere::new(0);
        let ray = Ray::new(
            Vec4d::new_point(0.0, 0.0, 0.0),
            Vec4d::new_vector(0.0, 0.0, 1.0),
        );

        let intersection_points = sphere.intersects(&ray);

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

        let intersection_points = sphere.intersects(&ray);

        assert_eq!(2, intersection_points.len());
        assert_eq!(-6.0, intersection_points[0].t);
        assert_eq!(-4.0, intersection_points[1].t);
    }
}
