mod display;
mod math;
mod matrix;
mod vec;

use display::{Canvas, Color};
use png_encode_mini;

use std::fs::File;

fn main() {
    let mut canvas = Canvas::new(100, 100);
    let mut file = match File::create("C:\\Users\\User\\Pictures\\crayfish_renders\\crayfish.png") {
        Ok(file) => file,
        Err(err) => panic!("Error creating file: {}", err),
    };

    let red = Color::new(255, 0, 0);
    for x in 30..=50 {
        for y in 30..=50 {
            canvas.set_pixel(x, y, &red);
        }
    }
    png_encode_mini::write_rgba_from_u8(
        &mut file,
        &canvas.data,
        canvas.width as u32,
        canvas.height as u32,
    )
    .unwrap();
}
