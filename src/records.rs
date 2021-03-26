use crate::defs::Real;
use crate::material::Material;
use crate::math::{Point3, Vector3};

pub struct IntersectionRecord<'record> {
    pub point: Point3,
    pub normal: Vector3,
    pub t: Real,
    pub front_face: bool,
    pub material: &'record Material,
}

impl<'record> IntersectionRecord<'record> {
    pub fn new(
        point: Point3,
        normal: Vector3,
        t: Real,
        front_face: bool,
        material: &'record Material,
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
