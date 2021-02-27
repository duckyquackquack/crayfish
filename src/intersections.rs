use crate::ray::Ray;
use crate::shapes::Shape;

use nalgebra::Vector4;

#[derive(Copy, Clone)]
pub struct IntersectionPoint<'a> {
    pub object: &'a dyn Shape,
    pub t: f64,
}

pub struct IntersectionComputation<'a> {
    pub object: &'a dyn Shape,
    pub t: f64,
    pub point: Vector4<f64>,
    pub to_eye: Vector4<f64>,
    pub normal: Vector4<f64>,
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
