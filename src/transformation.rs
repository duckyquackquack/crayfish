use crate::matrix::Matrix4d;

pub struct TransformBuilder {
    transforms: Vec<Matrix4d>,
}

impl TransformBuilder {
    pub fn new() -> TransformBuilder {
        TransformBuilder {
            transforms: Vec::new(),
        }
    }

    pub fn add_translation<'a>(
        &'a mut self,
        tx: f64,
        ty: f64,
        tz: f64,
    ) -> &'a mut TransformBuilder {
        let mut t = Matrix4d::identity();

        t.data[0][3] = tx;
        t.data[1][3] = ty;
        t.data[2][3] = tz;

        self.transforms.push(t);

        self
    }

    pub fn add_scale<'a>(&'a mut self, sx: f64, sy: f64, sz: f64) -> &'a mut TransformBuilder {
        let mut t = Matrix4d::identity();

        t.data[0][0] = sx;
        t.data[1][1] = sy;
        t.data[2][2] = sz;

        self.transforms.push(t);

        self
    }

    pub fn add_x_rotation<'a>(&'a mut self, rad: f64) -> &'a mut TransformBuilder {
        let mut t = Matrix4d::identity();

        let s = rad.sin();
        let c = rad.cos();

        t.data[1][1] = c;
        t.data[2][1] = s;
        t.data[1][2] = -s;
        t.data[2][2] = c;

        self.transforms.push(t);

        self
    }

    pub fn add_y_rotation<'a>(&'a mut self, rad: f64) -> &'a mut TransformBuilder {
        let mut t = Matrix4d::identity();

        let s = rad.sin();
        let c = rad.cos();

        t.data[0][0] = c;
        t.data[0][2] = s;
        t.data[2][0] = -s;
        t.data[2][2] = c;

        self.transforms.push(t);

        self
    }

    pub fn add_z_rotation<'a>(&'a mut self, rad: f64) -> &'a mut TransformBuilder {
        let mut t = Matrix4d::identity();

        let s = rad.sin();
        let c = rad.cos();

        t.data[0][0] = c;
        t.data[0][1] = -s;
        t.data[1][0] = s;
        t.data[1][1] = c;

        self.transforms.push(t);

        self
    }

    pub fn build(&self) -> Matrix4d {
        let mut copy = self.transforms.clone();
        copy.reverse();

        let mut m = Matrix4d::identity();
        for t in copy {
            m = m * t;
        }

        m
    }
}

#[cfg(test)]
mod transformation_tests {
    use super::TransformBuilder;
    use crate::vec::Vec4d;

    #[test]
    fn can_translate_a_point() {
        let m = TransformBuilder::new()
            .add_translation(5.0, -3.0, 2.0)
            .build();
        let p = Vec4d::new_point(-3.0, 4.0, 5.0);

        let result = m * p;
        let expected_result = Vec4d::new_point(2.0, 1.0, 7.0);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn transform_point_by_inverse_does_opposite_transform() {
        let m = TransformBuilder::new()
            .add_translation(5.0, -3.0, 2.0)
            .build();
        let p = Vec4d::new_point(-3.0, 4.0, 5.0);

        let result = m.inverse() * p;
        let expected_result = Vec4d::new_point(-8.0, 7.0, 3.0);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn cannot_translate_a_vector() {
        let m = TransformBuilder::new()
            .add_translation(5.0, -3.0, 2.0)
            .build();
        let p = Vec4d::new_vector(-3.0, 4.0, 5.0);

        let result = m * p;

        assert_eq!(result, p);
    }

    #[test]
    fn can_scale_a_point() {
        let m = TransformBuilder::new().add_scale(2.0, 3.0, 4.0).build();
        let p = Vec4d::new_point(-4.0, 6.0, 8.0);

        let result = m * p;
        let expected_result = Vec4d::new_point(-8.0, 18.0, 32.0);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_scale_a_vector() {
        let m = TransformBuilder::new().add_scale(2.0, 3.0, 4.0).build();
        let p = Vec4d::new_vector(-4.0, 6.0, 8.0);

        let result = m * p;
        let expected_result = Vec4d::new_vector(-8.0, 18.0, 32.0);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn scaling_point_by_inverse_does_opposote_transform() {
        let m = TransformBuilder::new().add_scale(2.0, 3.0, 4.0).build();
        let p = Vec4d::new_point(-4.0, 6.0, 8.0);

        let result = m.inverse() * p;
        let expected_result = Vec4d::new_point(-2.0, 2.0, 2.0);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_reflect_point_with_negative_scaling() {
        let m = TransformBuilder::new().add_scale(-1.0, 1.0, 1.0).build();
        let p = Vec4d::new_point(2.0, 3.0, 4.0);

        let result = m * p;
        let expected_result = Vec4d::new_point(-2.0, 3.0, 4.0);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_rotate_a_point_around_x_axis() {
        let pi = std::f64::consts::PI;
        let half_quarter_rotation = TransformBuilder::new().add_x_rotation(pi / 4.0).build();
        let full_quarter_rotation = TransformBuilder::new().add_x_rotation(pi / 2.0).build();

        let p = Vec4d::new_point(0.0, 1.0, 0.0);

        let result1 = half_quarter_rotation * p;
        let result2 = full_quarter_rotation * p;

        let expected_result1 = Vec4d::new_point(0.0, 0.5 * f64::sqrt(2.0), 0.5 * f64::sqrt(2.0));
        let expected_result2 = Vec4d::new_point(0.0, 0.00000000000000006123233995736766, 1.0);

        assert_eq!(result1, expected_result1);
        assert_eq!(result2, expected_result2);
    }

    #[test]
    fn can_rotate_a_point_around_y_axis() {
        let pi = std::f64::consts::PI;
        let half_quarter_rotation = TransformBuilder::new().add_y_rotation(pi / 4.0).build();
        let full_quarter_rotation = TransformBuilder::new().add_y_rotation(pi / 2.0).build();

        let p = Vec4d::new_point(0.0, 0.0, 1.0);

        let result1 = half_quarter_rotation * p;
        let result2 = full_quarter_rotation * p;

        let expected_result1 = Vec4d::new_point(0.5 * f64::sqrt(2.0), 0.0, 0.5 * f64::sqrt(2.0));
        let expected_result2 = Vec4d::new_point(1.0, 0.0, 0.00000000000000006123233995736766);

        assert_eq!(result1, expected_result1);
        assert_eq!(result2, expected_result2);
    }

    #[test]
    fn can_rotate_a_point_around_z_axis() {
        let pi = std::f64::consts::PI;
        let half_quarter_rotation = TransformBuilder::new().add_z_rotation(pi / 4.0).build();
        let full_quarter_rotation = TransformBuilder::new().add_z_rotation(pi / 2.0).build();

        let p = Vec4d::new_point(0.0, 1.0, 0.0);

        let result1 = half_quarter_rotation * p;
        let result2 = full_quarter_rotation * p;

        let expected_result1 = Vec4d::new_point(-0.5 * f64::sqrt(2.0), 0.5 * f64::sqrt(2.0), 0.0);
        let expected_result2 = Vec4d::new_point(-1.0, 0.00000000000000006123233995736766, 0.0);

        assert_eq!(result1, expected_result1);
        assert_eq!(result2, expected_result2);
    }

    #[test]
    fn can_chain_transformations() {
        let m = TransformBuilder::new()
            .add_x_rotation(std::f64::consts::PI * 0.5)
            .add_scale(5.0, 5.0, 5.0)
            .add_translation(10.0, 5.0, 7.0)
            .build();
        let p = Vec4d::new_point(1.0, 0.0, 1.0);

        let result = m * p;
        let expected_result = Vec4d::new_point(15.0, 0.0, 7.0);

        assert_eq!(result, expected_result);
    }
}
