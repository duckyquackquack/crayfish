use crate::matrix::Matrix4d;
use crate::vec::Vec4d;

struct TransformBuilder {
    transforms: Vec<Matrix4d>,
    is_built: bool,
}

impl TransformBuilder {
    pub fn new() -> TransformBuilder {
        TransformBuilder {
            transforms: Vec::new(),
            is_built: false
        }
    }

    pub fn add_translation(&mut self, translation: &Vec4d) -> &TransformBuilder {
        let mut t = Matrix4d::identity();

        t.data[0][3] = translation.x;
        t.data[1][3] = translation.y;
        t.data[2][3] = translation.z;
        
        self.transforms.push(t);

        self
    }

    pub fn add_scale(&mut self, scale: &Vec4d) -> &TransformBuilder {
        let mut t = Matrix4d::identity();

        t.data[0][0] = scale.x;
        t.data[1][1] = scale.y;
        t.data[2][2] = scale.z;
        
        self.transforms.push(t);

        self
    }

    pub fn add_x_rotation(&mut self, rad: f64) -> &TransformBuilder {
        let mut t = Matrix4d::identity();

        let s = rad.sin();
        let c = rad.cos();

        t.data[1][1] = c;
        t.data[2][1] = -s;
        t.data[1][2] = s;
        t.data[2][2] = c;
        
        self.transforms.push(t);

        self
    }

    pub fn add_y_rotation(&mut self, rad: f64) -> &TransformBuilder {
        let mut t = Matrix4d::new();

        let s = rad.sin();
        let c = rad.cos();

        t.data[0][0] = c;
        t.data[0][2] = s;
        t.data[2][0] = -s;
        t.data[2][2] = c;
        
        self.transforms.push(t);

        self
    }

    pub fn add_z_rotation(&mut self, rad: f64) -> &TransformBuilder {
        let mut t = Matrix4d::new();

        let s = rad.sin();
        let c = rad.cos();

        t.data[0][0] = c;
        t.data[0][1] = -s;
        t.data[1][0] = s;
        t.data[1][1] = c;
        
        self.transforms.push(t);

        self
    }

    pub fn build(&mut self) -> Matrix4d {
        if self.is_built {
            panic!("Transform has already been built, please use a new instance of TransformBuilder")
        }
        self.transforms.reverse();
        let mut m = Matrix4d::identity();
        for t in &self.transforms {
            m = m * *t;
        }
        self.is_built = true;
        m
    }
}

