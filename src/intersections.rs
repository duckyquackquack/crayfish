#[derive(Debug, Copy, Clone, PartialEq)]
pub struct IntersectionPoint {
    pub object_id: u32,
    pub t: f64,
}

impl IntersectionPoint {
    pub fn new(object_id: u32, t: f64) -> IntersectionPoint {
        IntersectionPoint { object_id, t }
    }
}
