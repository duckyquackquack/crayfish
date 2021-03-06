use crate::defs::Real;

use rand::{self, Rng};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub};
#[derive(Copy, Clone)]
pub struct Vector3 {
    data: [Real; 3],
}

impl Vector3 {
    pub fn new(x: Real, y: Real, z: Real) -> Vector3 {
        Vector3 { data: [x, y, z] }
    }

    pub fn new_random(min: Real, max: Real) -> Vector3 {
        let mut rng = rand::thread_rng();
        Vector3 {
            data: [
                rng.gen_range(min..=max),
                rng.gen_range(min..=max),
                rng.gen_range(min..=max),
            ],
        }
    }

    pub fn random_in_unit_sphere() -> Vector3 {
        loop {
            let random_vec = Self::new_random(-1.0, 1.0);
            if random_vec.magnitude_squared() >= 1.0 {
                continue;
            }
            return random_vec;
        }
    }

    pub fn random_in_unit_disk() -> Vector3 {
        loop {
            let mut p = Self::new_random(-1.0, 1.0);
            p.data[2] = 0.0;

            if p.magnitude_squared() >= 1.0 {
                continue;
            }

            return p;
        }
    }

    pub fn default() -> Vector3 {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn magnitude(&self) -> Real {
        Real::sqrt(self.magnitude_squared())
    }

    pub fn magnitude_squared(&self) -> Real {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }

    pub fn dot(&self, other: &Vector3) -> Real {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }

    pub fn as_normal(&self) -> Vector3 {
        self / self.magnitude()
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            data: [
                self[1] * other[2] - self[2] * other[1],
                self[2] * other[0] - self[0] * other[2],
                self[0] * other[1] - self[1] * other[0],
            ],
        }
    }

    pub fn is_near_zero(&self) -> bool {
        let epsilon: Real = 0.000001;

        self[0].abs() <= epsilon && self[1].abs() <= epsilon && self[2].abs() <= epsilon
    }

    pub fn reflect(&self, around: &Vector3) -> Vector3 {
        *self - (*around * 2.0 * self.dot(around))
    }

    pub fn refract(&self, normal: &Vector3, refraction_index: Real) -> Vector3 {
        let cos_theta = Real::min(-self.dot(&normal), 1.0);

        let perpendicular_component = (*self + (*normal * cos_theta)) * refraction_index;
        let parallel_component =
            *normal * -((1.0 - perpendicular_component.magnitude_squared()).abs()).sqrt();

        perpendicular_component + parallel_component
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3 {
            data: [-self[0], -self[1], -self[2]],
        }
    }
}

impl Index<usize> for Vector3 {
    type Output = Real;

    fn index(&self, i: usize) -> &Real {
        &self.data[i]
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            data: [
                self[0] + other.data[0],
                self[1] + other.data[1],
                self[2] + other.data[2],
            ],
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Vector3) {
        self.data[0] += other.data[0];
        self.data[1] += other.data[1];
        self.data[2] += other.data[2];
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            data: [
                self[0] - other.data[0],
                self[1] - other.data[1],
                self[2] - other.data[2],
            ],
        }
    }
}

impl Sub<Vector3> for &Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            data: [
                self[0] - other.data[0],
                self[1] - other.data[1],
                self[2] - other.data[2],
            ],
        }
    }
}

impl MulAssign for Vector3 {
    fn mul_assign(&mut self, other: Vector3) {
        self.data[0] *= other.data[0];
        self.data[1] *= other.data[1];
        self.data[2] *= other.data[2];
    }
}

impl Mul for Vector3 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {
            data: [self[0] * other[0], self[1] * other[1], self[2] * other[2]],
        }
    }
}

impl Mul<Real> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: Real) -> Vector3 {
        Vector3 {
            data: [self[0] * other, self[1] * other, self[2] * other],
        }
    }
}

impl Div<Real> for Vector3 {
    type Output = Vector3;

    fn div(self, other: Real) -> Vector3 {
        let inv = 1.0 / other;
        Vector3 {
            data: [self[0] * inv, self[1] * inv, self[2] * inv],
        }
    }
}

impl Div<Real> for &Vector3 {
    type Output = Vector3;

    fn div(self, other: Real) -> Vector3 {
        let inv = 1.0 / other;
        Vector3 {
            data: [self[0] * inv, self[1] * inv, self[2] * inv],
        }
    }
}

impl DivAssign for Vector3 {
    fn div_assign(&mut self, other: Vector3) {
        self.data[0] /= other.data[0];
        self.data[1] /= other.data[1];
        self.data[2] /= other.data[2];
    }
}

pub type Point3 = Vector3;
pub type Color3 = Vector3;

pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: Real) -> Point3 {
        self.origin + (self.direction * t)
    }
}
