use crate::matrix::Matrix4d;
use crate::vec::Vec4d;
use std::ops::Mul;

pub struct Ray {
    pub origin: Vec4d,
    pub direction: Vec4d,
}

impl Ray {
    pub fn new(origin: Vec4d, direction: Vec4d) -> Ray {
        Ray { origin, direction }
    }

    pub fn position_at(&self, t: f64) -> Vec4d {
        self.origin + (self.direction * t)
    }
}

impl Mul<Matrix4d> for Ray {
    type Output = Ray;

    fn mul(self, other: Matrix4d) -> Ray {
        Ray::new(other * self.origin, other * self.direction)
    }
}

impl Mul<Matrix4d> for &Ray {
    type Output = Ray;

    fn mul(self, other: Matrix4d) -> Ray {
        Ray::new(other * self.origin, other * self.direction)
    }
}

#[cfg(test)]
mod ray_tests {
    use super::Ray;
    use crate::transformation::TransformBuilder;
    use crate::vec::Vec4d;

    #[test]
    fn can_find_position_in_time() {
        let r = Ray::new(
            Vec4d::new_point(2.0, 3.0, 4.0),
            Vec4d::new_vector(1.0, 0.0, 0.0),
        );

        assert_eq!(r.position_at(0.0), r.origin);
        assert_eq!(r.position_at(1.0), Vec4d::new_point(3.0, 3.0, 4.0));
        assert_eq!(r.position_at(-1.0), Vec4d::new_point(1.0, 3.0, 4.0));
        assert_eq!(r.position_at(2.5), Vec4d::new_point(4.5, 3.0, 4.0));
    }

    #[test]
    fn translates_the_origin_but_not_the_direction() {
        let r = Ray::new(
            Vec4d::new_point(1.0, 2.0, 3.0),
            Vec4d::new_vector(0.0, 1.0, 0.0),
        );

        let m = TransformBuilder::new()
            .add_translation(3.0, 4.0, 5.0)
            .build();

        let result = r * m;

        assert_eq!(result.origin, Vec4d::new_point(4.0, 6.0, 8.0));
        assert_eq!(result.direction, Vec4d::new_vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn scales_the_origin_and_direction() {
        let r = Ray::new(
            Vec4d::new_point(1.0, 2.0, 3.0),
            Vec4d::new_vector(0.0, 1.0, 0.0),
        );

        let m = TransformBuilder::new().add_scale(2.0, 3.0, 4.0).build();

        let result = r * m;

        assert_eq!(result.origin, Vec4d::new_point(2.0, 6.0, 12.0));
        assert_eq!(result.direction, Vec4d::new_vector(0.0, 3.0, 0.0));
    }
}
