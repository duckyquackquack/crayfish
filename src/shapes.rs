use crate::defs::Real;
use crate::math::{Point3, Ray};
use crate::records::IntersectionRecord;

use crate::material::Material;

pub enum Shape {
    Sphere {
        center: Point3,
        radius: Real,
        material: Material,
    },
}

pub trait Intersectable {
    fn hit(&self, ray: &Ray, t_min: Real, t_max: Real) -> Option<IntersectionRecord>;
}

impl Intersectable for Shape {
    fn hit(&self, ray: &Ray, t_min: Real, t_max: Real) -> Option<IntersectionRecord> {
        match self {
            Shape::Sphere {
                center,
                radius,
                material,
            } => {
                let oc = ray.origin - *center;

                let a: Real = ray.direction.magnitude_squared();
                let half_b: Real = oc.dot(&ray.direction);
                let c: Real = oc.magnitude_squared() - radius * radius;

                let discriminant: Real = half_b * half_b - (a * c);

                if discriminant < 0.0 {
                    return None;
                }

                let disc_sqrt: Real = discriminant.sqrt();
                let mut root: Real = (-half_b - disc_sqrt) / a;
                if root < t_min || t_max < root {
                    root = (-half_b + disc_sqrt) / a;
                    if root < t_min || t_max < root {
                        return None;
                    }
                }

                let intersection_point = ray.at(root);
                let mut intersection_normal = (intersection_point - *center) / *radius;
                let front_face = ray.direction.dot(&intersection_normal) < 0.0;
                if !front_face {
                    intersection_normal = -intersection_normal;
                }
                let intersection = IntersectionRecord::new(
                    intersection_point,
                    intersection_normal,
                    root,
                    front_face,
                    material,
                );

                Some(intersection)
            }
        }
    }
}
