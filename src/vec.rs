use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec4d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vec4d {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Vec4d {
        Vec4d { x, y, z, w }
    }

    pub fn new_point(x: f64, y: f64, z: f64) -> Vec4d {
        Vec4d { x, y, z, w: 1.0 }
    }

    pub fn new_vector(x: f64, y: f64, z: f64) -> Vec4d {
        Vec4d { x, y, z, w: 0.0 }
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();
        self.x /= magnitude;
        self.y /= magnitude;
        self.z /= magnitude;
    }

    pub fn as_normalized(&self) -> Vec4d {
        let magnitude = self.magnitude();
        Vec4d::new_vector(self.x / magnitude, self.y / magnitude, self.z / magnitude)
    }

    pub fn dot(&self, other: &Vec4d) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: &Vec4d) -> Vec4d {
        Vec4d {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
            w: 0.0,
        }
    }

    pub fn reflect_around(&self, normal: &Vec4d) -> Vec4d {
        *self - *normal * 2.0 * self.dot(normal)
    }
}

impl Neg for Vec4d {
    type Output = Vec4d;

    fn neg(self) -> Vec4d {
        Vec4d {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Add for Vec4d {
    type Output = Vec4d;

    fn add(self, other: Vec4d) -> Vec4d {
        Vec4d {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Vec4d {
    type Output = Vec4d;

    fn sub(self, other: Vec4d) -> Vec4d {
        Vec4d {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Sub<&Vec4d> for Vec4d {
    type Output = Vec4d;

    fn sub(self, other: &Vec4d) -> Vec4d {
        Vec4d {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Mul<f64> for Vec4d {
    type Output = Vec4d;

    fn mul(self, other: f64) -> Vec4d {
        Vec4d {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl Div<f64> for Vec4d {
    type Output = Vec4d;

    fn div(self, other: f64) -> Vec4d {
        Vec4d {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

#[cfg(test)]
mod vec4d_tests {
    use super::Vec4d;

    #[test]
    fn is_vector_when_w_zero() {
        let vector = Vec4d::new_vector(1.0, 2.0, 3.0);

        assert_eq!(vector.is_vector(), true);
        assert_eq!(vector.is_point(), false);
    }

    #[test]
    fn is_point_when_w_one() {
        let point = Vec4d::new_point(1.0, 2.0, 3.0);

        assert_eq!(point.is_vector(), false);
        assert_eq!(point.is_point(), true);
    }

    #[test]
    fn can_assign_xyz() {
        let vector = Vec4d::new_vector(1.0, 2.0, 3.0);

        assert_eq!(vector.x, 1.0);
        assert_eq!(vector.y, 2.0);
        assert_eq!(vector.z, 3.0);
        assert_eq!(vector.w, 0.0);
    }

    #[test]
    fn can_compare_vecs() {
        let a = Vec4d::new_vector(1.0, 2.0, 3.0);
        let b = Vec4d::new_vector(1.0, 2.0, 3.0);
        let c = Vec4d::new_vector(1.0, 2.0, 4.0);

        let a_equal_to_b = a == b;
        let b_equal_to_c = b == c;

        assert_eq!(true, a_equal_to_b);
        assert_eq!(false, b_equal_to_c);
    }

    #[test]
    fn can_add_two_vec4d() {
        let a = Vec4d::new_vector(1.0, 2.0, 3.0);
        let b = Vec4d::new_point(4.0, 5.0, -2.0);

        let result = a + b;
        let expected_result = Vec4d::new_point(5.0, 7.0, 1.0);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_subtract_two_vec4d() {
        let a = Vec4d::new_vector(1.0, 2.0, 3.0);
        let b = Vec4d::new_point(4.0, 5.0, -2.0);

        let result = b - a;
        let expected_result = Vec4d::new_point(3.0, 3.0, -5.0);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn negate_flips_a_vec4d() {
        let a = Vec4d::new_vector(1.0, 2.0, 3.0);
        let expected_result = Vec4d::new_vector(-1.0, -2.0, -3.0);

        let result = -a;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_scale_vec4d() {
        let scale: f64 = 2.0;
        let a = Vec4d::new_point(1.0, 2.0, 3.0);

        let mut expected_result = Vec4d::new_point(2.0, 4.0, 6.0);
        expected_result.w = 2.0;

        let result = a * scale;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_divide_vec4d() {
        let scale: f64 = 2.0;
        let a = Vec4d::new_point(1.0, 2.0, 3.0);

        let mut expected_result = Vec4d::new_point(0.5, 1.0, 1.5);
        expected_result.w = 0.5;

        let result = a / scale;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn calculates_magnitude() {
        let a = Vec4d::new_point(1.0, 2.0, 3.0);
        let expected_result = f64::sqrt(14.0);
        let result = a.magnitude();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn normalizes_into_unit_length() {
        let mut a = Vec4d::new_point(1.0, 2.0, 3.0);
        let b = a.as_normalized();
        a.normalize();

        assert_eq!(a.magnitude(), 1.0);
        assert_eq!(b.magnitude(), 1.0);
    }

    #[test]
    fn computes_dot_product() {
        let a = Vec4d::new_vector(1.0, 2.0, 3.0);
        let b = Vec4d::new_vector(2.0, 3.0, 4.0);

        let expected_result = 20.0;
        let result = a.dot(&b);

        assert_eq!(expected_result, result);
    }

    #[test]
    fn computes_cross_product() {
        let a = Vec4d::new_vector(1.0, 2.0, 3.0);
        let b = Vec4d::new_vector(2.0, 3.0, 4.0);

        let expected_result = Vec4d::new_vector(-1.0, 2.0, -1.0);
        let result = a.cross(&b);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn reflects_on_flat_surface() {
        let v = Vec4d::new_vector(1.0, -1.0, 0.0);
        let n = Vec4d::new_vector(0.0, 1.0, 0.0);

        let result = v.reflect_around(&n);

        assert_eq!(result, Vec4d::new_vector(1.0, 1.0, 0.0));
    }

    #[test]
    fn relects_on_slanted_surface() {
        let v = Vec4d::new_vector(0.0, -1.0, 0.0);
        let n = Vec4d::new_vector(f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0, 0.0);

        let result = v.reflect_around(&n);
        let expected_result = Vec4d::new_vector(1.0, 0.0, 0.0);

        assert!((result.x - expected_result.x).abs() < 0.0001);
        assert!((result.y - expected_result.y).abs() < 0.0001);
        assert!((result.z - expected_result.z).abs() < 0.0001);
        assert!((result.w - expected_result.w).abs() < 0.0001);
    }
}
