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
        self.data[y * self.width + x] = *color;
    }

    pub fn to_u8_vec(&self) -> Vec<u8> {
        let mut u8_vec = Vec::with_capacity(self.data.len() * 4);
        for color in self.data.iter() {
            u8_vec.push((color.r.min(1.0) * 255.0) as u8);
            u8_vec.push((color.g.min(1.0) * 255.0) as u8);
            u8_vec.push((color.b.min(1.0) * 255.0) as u8);
            u8_vec.push(255);
        }

        u8_vec
    }
}

#[cfg(test)]
mod canvas_tests {
    use super::{Canvas, Color};

    #[test]
    fn constructs_canvas_of_given_size() {
        let width = 64;
        let height = 32;

        let canvas = Canvas::new(width, height);

        assert_eq!(canvas.height, height);
        assert_eq!(canvas.width, width);
        assert_eq!(canvas.data.len(), width * height);
    }

    #[test]
    fn writes_color_to_canvas_at_given_position() {
        let color = Color::new(1.0, 0.0, 0.0);

        let width = 64;
        let height = 32;
        let mut canvas = Canvas::new(width, height);

        let x = 10;
        let y = 15;

        canvas.set_pixel(x, y, &color);

        assert_eq!(canvas.data[y * width + x], color);
    }
}
