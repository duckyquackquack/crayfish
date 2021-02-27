use nalgebra::{Matrix4, Vector3};

pub struct TransformBuilder {
    transforms: Vec<Matrix4<f64>>,
}

impl TransformBuilder {
    pub fn new() -> TransformBuilder {
        TransformBuilder {
            transforms: Vec::new(),
        }
    }

    pub fn add_translation(&mut self, tx: f64, ty: f64, tz: f64) -> &mut TransformBuilder {
        self.transforms
            .push(Matrix4::new_translation(&Vector3::new(tx, ty, tz)));
        self
    }

    pub fn add_scale(&mut self, sx: f64, sy: f64, sz: f64) -> &mut TransformBuilder {
        self.transforms
            .push(Matrix4::new_nonuniform_scaling(&Vector3::new(sx, sy, sz)));
        self
    }

    pub fn add_x_rotation(&mut self, rad: f64) -> &mut TransformBuilder {
        self.transforms
            .push(Matrix4::new_rotation(Vector3::new(rad, 0.0, 0.0)));
        self
    }

    pub fn add_y_rotation(&mut self, rad: f64) -> &mut TransformBuilder {
        self.transforms
            .push(Matrix4::new_rotation(Vector3::new(0.0, rad, 0.0)));
        self
    }

    pub fn add_z_rotation(&mut self, rad: f64) -> &mut TransformBuilder {
        self.transforms
            .push(Matrix4::new_rotation(Vector3::new(0.0, 0.0, rad)));
        self
    }

    pub fn build(&self) -> Matrix4<f64> {
        let mut copy = self.transforms.clone();
        copy.reverse();

        let mut m = Matrix4::identity();
        for t in copy {
            m = m * t;
        }

        m
    }
}
