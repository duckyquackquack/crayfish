#[derive(Debug)]
pub struct Vec2d {
    x: f64,
    y: f64,
}

impl Vec2d {
    pub fn new(x: f64, y: f64) -> Self {
        Vec2d { x, y }
    }
}

#[derive(Debug)]
pub struct Vec3d {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3d { x, y, z }
    }
}

#[derive(Debug)]
pub struct Vec4d {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Vec4d {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Vec4d { x, y, z, w }
    }
}
