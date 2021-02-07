mod math;

use math::Vec4d;

fn main() {
    let something_else = Vec4d::new_point(1.0, 2.0, 3.0);
    println!("{:?}", something_else);
}
