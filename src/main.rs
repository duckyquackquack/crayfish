mod math;

use math::{Vec2d, Vec3d, Vec4d};

fn main() {
    let pos = Vec2d::new(100.0, 200.0);
    let vel = Vec3d::new(1.0, 2.0, 3.0);
    let something_else = Vec4d::new(1.0, 2.0, 3.0, 1.0);
    println!(
        "position is: {:?}. velocity is: {:?}. something else is {:?}",
        pos, vel, something_else
    );
}
