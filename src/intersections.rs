use crate::ray::Ray;
use crate::shapes::Shape;
use crate::vec::Vec4d;

#[derive(Copy, Clone)]
pub struct IntersectionPoint<'a> {
    pub object: &'a dyn Shape,
    pub t: f64,
}

pub struct IntersectionComputation<'a> {
    pub object: &'a dyn Shape,
    pub t: f64,
    pub point: Vec4d,
    pub to_eye: Vec4d,
    pub normal: Vec4d,
    pub inside: bool,
}

impl IntersectionPoint<'_> {
    pub fn new(object: &dyn Shape, t: f64) -> IntersectionPoint {
        IntersectionPoint { object, t }
    }

    pub fn prepare_computation(&self, ray: &Ray) -> IntersectionComputation {
        let point = ray.position_at(self.t);
        let to_eye = -ray.direction;
        let mut normal = self.object.normal_at(&point);
        let mut inside: bool = false;

        if normal.dot(&to_eye) < 0.0 {
            normal = -normal;
            inside = true;
        }

        IntersectionComputation {
            object: self.object,
            t: self.t,
            point,
            to_eye,
            normal,
            inside,
        }
    }
}

#[cfg(test)]
mod intersection_tests {
    use super::*;
    use crate::ray::Ray;
    use crate::shapes::*;
    use crate::vec::Vec4d;

    #[test]
    fn hit_that_occurs_on_outside() {
        let ray = Ray::new(
            Vec4d::new_point(0.0, 0.0, 5.0),
            Vec4d::new_vector(0.0, 0.0, 1.0),
        );
        let sphere = Sphere::new();
        let intersections = sphere.intersections(&ray);

        let intersection_computation = intersections[0].prepare_computation(&ray);

        assert_eq!(false, intersection_computation.inside);
    }

    #[test]
    fn hit_that_occurs_on_inside() {
        let ray = Ray::new(
            Vec4d::new_point(0.0, 0.0, 0.0),
            Vec4d::new_vector(0.0, 0.0, 1.0),
        );
        let sphere = Sphere::new();
        let intersections = sphere.intersections(&ray);

        let intersection_computation = intersections[1].prepare_computation(&ray);

        assert_eq!(true, intersection_computation.inside);
        assert_eq!(
            Vec4d::new_point(0.0, 0.0, 1.0),
            intersection_computation.point
        );
        assert_eq!(
            Vec4d::new_vector(0.0, 0.0, -1.0),
            intersection_computation.to_eye
        );
        assert_eq!(
            Vec4d::new_vector(0.0, 0.0, -1.0),
            intersection_computation.normal
        );
    }
}
