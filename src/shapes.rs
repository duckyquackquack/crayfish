use crate::defs::Real;
use crate::math::{Point3, Ray};
use crate::records::IntersectionRecord;

use crate::material::Material;
use std::rc::Rc;

pub trait Shape {
    fn hit(&self, ray: &Ray, t_min: Real, t_max: Real) -> IntersectionRecord;
}

pub struct Sphere {
    center: Point3,
    radius: Real,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: Real, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Shape for Sphere {
    fn hit(&self, ray: &Ray, t_min: Real, t_max: Real) -> IntersectionRecord {
        let oc = ray.origin - self.center;

        let a: Real = ray.direction.magnitude_squared();
        let half_b: Real = oc.dot(&ray.direction);
        let c: Real = oc.magnitude_squared() - self.radius * self.radius;

        let discriminant: Real = half_b * half_b - (a * c);

        if discriminant < 0.0 {
            return IntersectionRecord::default();
        }

        let disc_sqrt: Real = discriminant.sqrt();
        let mut root: Real = (-half_b - disc_sqrt) / a;
        if root < t_min || root > t_max {
            root = (-half_b + disc_sqrt) / a;
            if root < t_min || root > t_max {
                return IntersectionRecord::default();
            }
        }

        let mut intersection = IntersectionRecord::default();
        intersection.hit = true;
        intersection.t = root;
        intersection.point = ray.at(root);
        intersection.normal = (intersection.point - self.center) / self.radius;
        intersection.front_face = ray.direction.dot(&intersection.normal) < 0.0;
        if !intersection.front_face {
            intersection.normal = -intersection.normal;
        }
        intersection.material = self.material.clone();

        intersection
    }
}
