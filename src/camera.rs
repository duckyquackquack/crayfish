use crate::defs::Real;
use crate::math::{Point3, Ray, Vector3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vector3,
    vertical: Vector3,
    lens_radius: Real,
    u: Vector3,
    v: Vector3,
}

impl Camera {
    pub fn new(
        origin: Point3,
        look_at: Point3,
        up: Vector3,
        aspect_ratio: Real,
        fov_deg: Real,
        focus_distance: Real,
        aperture: Real,
    ) -> Camera {
        let theta = fov_deg.to_radians();
        let h = (theta * 0.5).tan();

        let viewport_height: Real = 2.0 * h;
        let viewport_width: Real = aspect_ratio * viewport_height;

        let w = (origin - look_at).as_normal();
        let u = up.cross(&w).as_normal();
        let v = w.cross(&u);

        let horizontal = u * viewport_width * focus_distance;
        let vertical = v * viewport_height * focus_distance;
        let lower_left_corner =
            origin - (horizontal / 2.0) - (vertical / 2.0) - (w * focus_distance);

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            lens_radius: aperture * 0.5,
            u,
            v,
        }
    }

    pub fn get_ray(&self, px: Real, py: Real) -> Ray {
        let random_disk = Vector3::random_in_unit_disk() * self.lens_radius;
        let offset = (self.u * random_disk[0]) + (self.v * random_disk[1]);

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + (self.horizontal * px) + (self.vertical * py)
                - self.origin
                - offset,
        )
    }
}
