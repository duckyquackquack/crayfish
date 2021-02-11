mod display;
mod math;
mod matrix;
mod vec;
mod transformation;

use display::{Canvas, Color};
use png_encode_mini;

use std::fs::File;

fn main() {
    // let mut canvas = Canvas::new(1920, 1080);
    // let mut file = match File::create("C:\\Users\\User\\Pictures\\crayfish_renders\\crayfish.png") {
    //     Ok(file) => file,
    //     Err(err) => panic!("Error creating file: {}", err),
    // };

    // let red = Color::new(255, 0, 0);
    // for x in 300..=500 {
    //     for y in 300..=500 {
    //         canvas.set_pixel(x, y, &red);
    //     }
    // }

    // let canvas_color_u8 = canvas.to_u8_vec();

    // png_encode_mini::write_rgba_from_u8(
    //     &mut file,
    //     &canvas_color_u8,
    //     canvas.width as u32,
    //     canvas.height as u32,
    // )
    // .unwrap();
    // //println!("{:?}", canvas);
}
