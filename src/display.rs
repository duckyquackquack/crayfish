use crate::color::Color;

#[derive(Debug)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    data: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            data: vec![Color::new(52.0 / 255.0, 198.0 / 255.0, 235.0 / 255.0); width * height],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: &Color) {
        self.data[(self.height as i32 - 1 - y as i32).abs() as usize * self.width + x] = *color;
    }

    pub fn to_u8_vec(&self) -> Vec<u8> {
        let mut u8_vec = Vec::with_capacity(self.data.len() * 3);

        for color in self.data.iter() {
            u8_vec.push((color.r.min(1.0) * 255.0) as u8);
            u8_vec.push((color.g.min(1.0) * 255.0) as u8);
            u8_vec.push((color.b.min(1.0) * 255.0) as u8);
        }

        u8_vec
    }
}
