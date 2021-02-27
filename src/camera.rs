// use crate::matrix::Matrix4d;
// use crate::transformation::TransformBuilder;
// use crate::vec::Vec4d;

// pub struct Camera {
//     width: usize,
//     height: usize,
//     fov: f64,
//     transform: Matrix4d,
// }

// impl Camera {
//     pub fn new(width: usize, height: usize, fov: f64) -> Camera {
//         Camera {
//             width,
//             height,
//             fov,
//             transform: Matrix4d::identity(),
//         }
//     }

//     fn view_transform(from: &Vec4d, to: &Vec4d, up: &Vec4d) -> Matrix4d {
//         let forward = (*to - from).as_normalized();
//         let up_n = up.as_normalized();
//         let left = forward.cross(&up_n);
//         let true_up = left.cross(&forward);

//         let mut m = Matrix4d::new();

//         m.data[0][0] = left.x;
//         m.data[1][0] = left.y;
//         m.data[2][0] = left.z;

//         m.data[0][1] = true_up.x;
//         m.data[1][1] = true_up.y;
//         m.data[2][1] = true_up.z;

//         m.data[0][2] = -forward.x;
//         m.data[1][2] = -forward.y;
//         m.data[2][2] = -forward.z;

//         m.data[3][3] = 1.0;

//         let translation = TransformBuilder::new()
//             .add_translation(-from.x, -from.y, -from.z)
//             .build();

//         translation * m
//     }
// }
