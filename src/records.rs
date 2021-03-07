use crate::defs::Real;
use crate::material::{DefaultMaterial, Material};
use crate::math::{Point3, Vector3};

use std::rc::Rc;

pub struct IntersectionRecord {
    pub point: Point3,
    pub normal: Vector3,
    pub t: Real,
    pub front_face: bool,
    pub hit: bool,
    pub material: Rc<dyn Material>,
}

impl IntersectionRecord {
    pub fn default() -> IntersectionRecord {
        IntersectionRecord {
            point: Point3::default(),
            normal: Vector3::default(),
            t: 0.0,
            front_face: false,
            hit: false,
            material: Rc::new(DefaultMaterial::new()),
        }
    }
}
