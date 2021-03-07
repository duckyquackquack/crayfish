use crate::defs::Real;
use crate::math::{Point3, Ray, Vector3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vector3,
    vertical: Vector3,
}

impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio: Real = 16.0 / 9.0;

        let viewport_height: Real = 2.0;
        let viewport_width: Real = aspect_ratio * viewport_height;
        let focal_length: Real = 1.0;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
        let vertical = Vector3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - (horizontal / 2.0) - (vertical / 2.0) - Vector3::new(0.0, 0.0, focal_length);

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, px: Real, py: Real) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + (self.horizontal * px) + (self.vertical * py) - self.origin,
        )
    }
}
