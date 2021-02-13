use crate::vec::Vec4d;

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

#[cfg(test)]
mod ray_tests {
    use super::Ray;
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
}
