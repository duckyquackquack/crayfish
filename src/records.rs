use crate::defs::Real;
use crate::material::Material;
use crate::math::{Point3, Vector3};

use std::rc::Rc;

pub struct IntersectionRecord {
    pub point: Point3,
    pub normal: Vector3,
    pub t: Real,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

impl IntersectionRecord {
    pub fn new(
        point: Point3,
        normal: Vector3,
        t: Real,
        front_face: bool,
        material: Rc<dyn Material>,
    ) -> Self {
        Self {
            point,
            normal,
            t,
            front_face,
            material,
        }
    }
}
