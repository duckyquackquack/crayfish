use nalgebra::{Matrix4, Vector4};
use std::ops::Mul;

pub struct Ray {
    pub origin: Vector4<f64>,
    pub direction: Vector4<f64>,
}

impl Ray {
    pub fn new(origin: Vector4<f64>, direction: Vector4<f64>) -> Ray {
        Ray { origin, direction }
    }

    pub fn position_at(&self, t: f64) -> Vector4<f64> {
        self.origin + (self.direction * t)
    }
}

impl Mul<Matrix4<f64>> for Ray {
    type Output = Ray;

    fn mul(self, other: Matrix4<f64>) -> Ray {
        Ray::new(other * self.origin, other * self.direction)
    }
}

impl Mul<Matrix4<f64>> for &Ray {
    type Output = Ray;

    fn mul(self, other: Matrix4<f64>) -> Ray {
        Ray::new(other * self.origin, other * self.direction)
    }
}
