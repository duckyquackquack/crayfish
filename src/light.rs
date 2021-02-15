use crate::color::Color;
use crate::vec::Vec4d;

pub struct PointLight {
    intensity: Color,
    position: Vec4d,
}

impl PointLight {
    pub fn new(intensity: Color, position: Vec4d) -> PointLight {
        PointLight {
            intensity,
            position,
        }
    }
}
