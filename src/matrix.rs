use std::ops::Mul;

use crate::vec::Vec4d;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Matrix4d {
    pub data: [[f64; 4]; 4],
}

#[derive(Debug, PartialEq)]
pub struct Matrix3d {
    data: [[f64; 3]; 3],
}

#[derive(Debug, PartialEq)]
pub struct Matrix2d {
    data: [[f64; 2]; 2],
}

impl Matrix4d {
    pub fn new() -> Matrix4d {
        Matrix4d {
            data: [[0.0; 4]; 4],
        }
    }

    pub fn identity() -> Matrix4d {
        let mut matrix = Matrix4d::new();
        for x in 0..=3 {
            matrix.data[x][x] = 1.0;
        }
        matrix
    }

    pub fn transpose(&self) -> Matrix4d {
        let mut matrix = Matrix4d::new();

        for x in 0..=3 {
            for y in 0..=3 {
                matrix.data[y][x] = self.data[x][y];
            }
        }

        matrix
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix3d {
        let mut matrix = Matrix3d::new();

        let mut current_3d_row = 0;
        let mut current_3d_col = 0;

        let mut added_to_current_row = false;

        for x in 0..=3 {
            for y in 0..=3 {
                if y != col && x != row {
                    matrix.data[current_3d_row][current_3d_col] = self.data[x][y];
                    current_3d_col += 1;
                    added_to_current_row = true;
                }
            }
            if added_to_current_row {
                current_3d_row += 1;
            }
            added_to_current_row = false;
            current_3d_col = 0;
        }

        matrix
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        match (row + col) % 2 {
            1 => -1.0 * minor,
            _ => minor,
        }
    }

    pub fn determinant(&self) -> f64 {
        let mut determinant = 0.0;
        for y in 0..=3 {
            determinant += self.data[0][y] * self.cofactor(0, y);
        }
        determinant
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn inverse(&self) -> Matrix4d {
        if self.is_invertible() {
            let mut inverse = Matrix4d::new();
            let determinant = self.determinant();

            for x in 0..=3 {
                for y in 0..=3 {
                    let cofactor = self.cofactor(x, y);
                    inverse.data[y][x] = cofactor / determinant;
                }
            }

            return inverse;
        }
        panic!("Cannot inverse matrix: {:?}", *self);
    }
}

impl Mul<Matrix4d> for Matrix4d {
    type Output = Matrix4d;

    fn mul(self, other: Matrix4d) -> Matrix4d {
        let mut matrix = Matrix4d::new();

        for x in 0..=3 {
            for y in 0..=3 {
                matrix.data[x][y] = (self.data[x][0] * other.data[0][y])
                    + (self.data[x][1] * other.data[1][y])
                    + (self.data[x][2] * other.data[2][y])
                    + (self.data[x][3] * other.data[3][y])
            }
        }

        matrix
    }
}

impl Mul<Vec4d> for Matrix4d {
    type Output = Vec4d;

    fn mul(self, other: Vec4d) -> Vec4d {
        Vec4d {
            x: (other.x * self.data[0][0])
                + (other.y * self.data[0][1])
                + (other.z * self.data[0][2] + other.w * self.data[0][3]),
            y: (other.x * self.data[1][0])
                + (other.y * self.data[1][1])
                + (other.z * self.data[1][2] + other.w * self.data[1][3]),
            z: (other.x * self.data[2][0])
                + (other.y * self.data[2][1])
                + (other.z * self.data[2][2] + other.w * self.data[2][3]),
            w: (other.x * self.data[3][0])
                + (other.y * self.data[3][1])
                + (other.z * self.data[3][2] + other.w * self.data[3][3]),
        }
    }
}

impl Matrix3d {
    pub fn new() -> Matrix3d {
        Matrix3d {
            data: [[0.0; 3]; 3],
        }
    }

    pub fn identity() -> Matrix3d {
        let mut matrix = Matrix3d::new();
        for x in 0..=2 {
            matrix.data[x][x] = 1.0;
        }
        matrix
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix2d {
        let mut matrix = Matrix2d::new();

        let mut current_2d_row = 0;
        let mut current_2d_col = 0;

        let mut added_to_current_row = false;

        for x in 0..=2 {
            for y in 0..=2 {
                if y != col && x != row {
                    matrix.data[current_2d_row][current_2d_col] = self.data[x][y];
                    current_2d_col += 1;
                    added_to_current_row = true;
                }
            }
            if added_to_current_row {
                current_2d_row += 1;
            }
            added_to_current_row = false;
            current_2d_col = 0;
        }

        matrix
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        match (row + col) % 2 {
            1 => -1.0 * minor,
            _ => minor,
        }
    }

    pub fn determinant(&self) -> f64 {
        let mut determinant = 0.0;
        for y in 0..=2 {
            determinant += self.data[0][y] * self.cofactor(0, y);
        }
        determinant
    }
}

impl Matrix2d {
    pub fn new() -> Matrix2d {
        Matrix2d {
            data: [[0.0; 2]; 2],
        }
    }

    pub fn identity() -> Matrix2d {
        let mut matrix = Matrix2d::new();
        for x in 0..=1 {
            matrix.data[x][x] = 1.0;
        }
        matrix
    }

    pub fn determinant(&self) -> f64 {
        (self.data[0][0] * self.data[1][1]) - (self.data[1][0] * self.data[0][1])
    }
}

#[cfg(test)]
mod matrix4d_tests {
    use super::{Matrix3d, Matrix4d};
    use crate::vec::Vec4d;

    #[test]
    fn can_construct_empty_4d_matrix() {
        let matrix = Matrix4d::new();
        let expected_result = [
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ];

        assert_eq!(matrix.data, expected_result);
    }

    #[test]
    fn can_construct_4d_identity_matrix() {
        let identity_matrix = Matrix4d::identity();
        let expected_result = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        assert_eq!(identity_matrix.data, expected_result);
    }

    #[test]
    fn can_compare_matrices() {
        let mut a = Matrix4d::new();
        a.data[0][0] = 10.0;

        let mut b = Matrix4d::new();
        b.data[0][0] = 10.0;

        let mut c = Matrix4d::new();
        c.data[0][0] = 9.0;

        assert_eq!(true, a == b);
        assert_eq!(false, b == c);
    }

    #[test]
    fn can_multiply_matrices() {
        let mut a = Matrix4d::new();
        a.data = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ];

        let mut b = Matrix4d::new();
        b.data = [
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ];

        let mut expected_result = Matrix4d::new();
        expected_result.data = [
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ];

        let result = a * b;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_multiple_with_vec() {
        let v = Vec4d::new_point(1.0, 2.0, 3.0);
        let mut m = Matrix4d::new();

        m.data = [
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        let result = m * v;

        let expected_result = Vec4d::new(18.0, 24.0, 33.0, 1.0);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn multiplying_identity_does_not_change_result() {
        let mut a = Matrix4d::new();
        a.data = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 0.0, 1.0, 2.0],
            [3.0, 4.0, 5.0, 6.0],
        ];

        let b = Matrix4d::identity();
        let result = a * b;

        assert_eq!(a, result);
    }

    #[test]
    fn multiplying_identity_with_vec_does_not_change_result() {
        let a = Vec4d::new(1.0, 2.0, 3.0, 4.0);
        let b = Matrix4d::identity();

        let result = b * a;
        assert_eq!(a, result);
    }

    #[test]
    fn transposes_correctly() {
        let mut a = Matrix4d::new();
        a.data = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 0.0, 1.0, 2.0],
            [3.0, 4.0, 5.0, 6.0],
        ];

        let transpose = a.transpose();

        let mut expected_result = Matrix4d::new();
        expected_result.data = [
            [1.0, 5.0, 9.0, 3.0],
            [2.0, 6.0, 0.0, 4.0],
            [3.0, 7.0, 1.0, 5.0],
            [4.0, 8.0, 2.0, 6.0],
        ];

        assert_eq!(transpose, expected_result);
    }

    #[test]
    fn can_get_3d_submatrix() {
        let mut a = Matrix4d::new();
        a.data = [
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ];

        let mut expected_result = Matrix3d::new();
        expected_result.data = [[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]];

        assert_eq!(a.submatrix(2, 1), expected_result);
    }

    #[test]
    fn calculates_determinant() {
        let mut a = Matrix4d::new();
        a.data = [
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ];

        let expected_result = -4071.0;

        assert_eq!(a.determinant(), expected_result);
    }

    #[test]
    fn calculates_inverse() {
        let mut a = Matrix4d::new();
        a.data = [
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ];

        let mut b = Matrix4d::new();
        b.data = [
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ];

        let mut c = Matrix4d::new();
        c.data = [
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        ];

        let mut expected_inverse_a = Matrix4d::new();
        expected_inverse_a.data = [
            [
                0.21804511278195488,
                0.45112781954887216,
                0.24060150375939848,
                -0.045112781954887216,
            ],
            [
                -0.8082706766917294,
                -1.4567669172932332,
                -0.44360902255639095,
                0.5206766917293233,
            ],
            [
                -0.07894736842105263,
                -0.2236842105263158,
                -0.05263157894736842,
                0.19736842105263158,
            ],
            [
                -0.5225563909774437,
                -0.8139097744360902,
                -0.3007518796992481,
                0.30639097744360905,
            ],
        ];

        let mut expected_inverse_b = Matrix4d::new();
        expected_inverse_b.data = [
            [
                -0.15384615384615385,
                -0.15384615384615385,
                -0.28205128205128205,
                -0.5384615384615384,
            ],
            [
                -0.07692307692307693,
                0.12307692307692308,
                0.02564102564102564,
                0.03076923076923077,
            ],
            [
                0.358974358974359,
                0.358974358974359,
                0.4358974358974359,
                0.9230769230769231,
            ],
            [
                -0.6923076923076923,
                -0.6923076923076923,
                -0.7692307692307693,
                -1.9230769230769231,
            ],
        ];

        let mut expected_inverse_c = Matrix4d::new();
        expected_inverse_c.data = [
            [
                -0.040740740740740744,
                -0.07777777777777778,
                0.14444444444444443,
                -0.2222222222222222,
            ],
            [
                -0.07777777777777778,
                0.03333333333333333,
                0.36666666666666664,
                -0.3333333333333333,
            ],
            [
                -0.029012345679012345,
                -0.14629629629629629,
                -0.10925925925925926,
                0.12962962962962962,
            ],
            [
                0.17777777777777778,
                0.06666666666666667,
                -0.26666666666666666,
                0.3333333333333333,
            ],
        ];

        assert_eq!(a.inverse(), expected_inverse_a);
        assert_eq!(b.inverse(), expected_inverse_b);
        assert_eq!(c.inverse(), expected_inverse_c);
    }
}

#[cfg(test)]
mod matrix3d_tests {
    use super::{Matrix2d, Matrix3d};

    #[test]
    fn can_construct_empty_3d_matrix() {
        let matrix = Matrix3d::new();
        let expected_result = [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]];

        assert_eq!(matrix.data, expected_result);
    }

    #[test]
    fn can_construct_3d_identity_matrix() {
        let identity_matrix = Matrix3d::identity();
        let expected_result = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

        assert_eq!(identity_matrix.data, expected_result);
    }

    #[test]
    fn can_compare_matrices() {
        let mut a = Matrix3d::new();
        a.data[0][0] = 10.0;

        let mut b = Matrix3d::new();
        b.data[0][0] = 10.0;

        let mut c = Matrix3d::new();
        c.data[0][0] = 9.0;

        assert_eq!(true, a == b);
        assert_eq!(false, b == c);
    }

    #[test]
    fn can_get_2d_submatrix() {
        let mut a = Matrix3d::new();
        a.data = [[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, 3.0]];

        let mut expected_result = Matrix2d::new();
        expected_result.data = [[-3.0, 2.0], [0.0, 6.0]];

        assert_eq!(a.submatrix(0, 2), expected_result);
    }

    #[test]
    fn calculates_minor() {
        let mut a = Matrix3d::new();
        a.data = [[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]];

        let expected_result = 25.0;

        assert_eq!(a.minor(1, 0), expected_result);
    }

    #[test]
    fn calculates_cofactor() {
        let mut a = Matrix3d::new();
        a.data = [[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]];

        assert_eq!(-12.0, a.cofactor(0, 0));
        assert_eq!(-25.0, a.cofactor(1, 0));
    }

    #[test]
    fn calculates_determinant() {
        let mut a = Matrix3d::new();
        a.data = [[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]];

        let expected_result = -196.0;

        assert_eq!(a.determinant(), expected_result);
    }
}

#[cfg(test)]
mod matrix2d_tests {
    use super::Matrix2d;

    #[test]
    fn can_construct_empty_2d_matrix() {
        let matrix = Matrix2d::new();
        let expected_result = [[0.0, 0.0], [0.0, 0.0]];

        assert_eq!(matrix.data, expected_result);
    }

    #[test]
    fn can_construct_2d_identity_matrix() {
        let identity_matrix = Matrix2d::identity();
        let expected_result = [[1.0, 0.0], [0.0, 1.0]];

        assert_eq!(identity_matrix.data, expected_result);
    }

    #[test]
    fn can_compare_matrices() {
        let mut a = Matrix2d::new();
        a.data[0][0] = 10.0;

        let mut b = Matrix2d::new();
        b.data[0][0] = 10.0;

        let mut c = Matrix2d::new();
        c.data[0][0] = 9.0;

        assert_eq!(true, a == b);
        assert_eq!(false, b == c);
    }

    #[test]
    fn matrid2d_determinant() {
        let mut a = Matrix2d::new();
        a.data = [[1.0, 5.0], [-3.0, 2.0]];

        let determinant = a.determinant();
        let expected_result = 17.0;

        assert_eq!(determinant, expected_result);
    }
}
