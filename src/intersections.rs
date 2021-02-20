use crate::shapes::Shape;

#[derive(Copy, Clone)]
pub struct IntersectionPoint<'a> {
    pub object: &'a dyn Shape,
    pub t: f64,
}

impl IntersectionPoint<'_> {
    pub fn new(object: &dyn Shape, t: f64) -> IntersectionPoint {
        IntersectionPoint { object, t }
    }
}
