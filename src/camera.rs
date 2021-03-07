use crate::defs::Real;
use crate::math::{Point3, Ray, Vector3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vector3,
    vertical: Vector3,
}

impl Camera {
    pub fn default() -> Camera {
        Camera::new(
            Point3::default(),
            Point3::new(0.0, 0.0, -1.0),
            Vector3::new(0.0, 1.0, 0.0),
            16.0 / 9.0,
            90.0,
        )
    }

    pub fn new(
        origin: Point3,
        look_at: Point3,
        up: Vector3,
        aspect_ratio: Real,
        fov_deg: Real,
    ) -> Camera {
        let theta = fov_deg.to_radians();
        let h = (theta * 0.5).tan();

        let viewport_height: Real = 2.0 * h;
        let viewport_width: Real = aspect_ratio * viewport_height;

        let w = (origin - look_at).as_normal();
        let u = up.cross(&w).as_normal();
        let v = w.cross(&u);

        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - w;

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
