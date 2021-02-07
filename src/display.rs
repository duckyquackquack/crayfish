#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

const NUM_COLOR_COMPONENTS: usize = 4;

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
    }
}

#[derive(Debug)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            data: vec![0; width * height * NUM_COLOR_COMPONENTS],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: &Color) {
        self.data[y * self.width + x + 0] = color.r;
        self.data[y * self.width + x + 1] = color.g;
        self.data[y * self.width + x + 2] = color.b;
        self.data[y * self.width + x + 3] = color.a;
    }
}

#[cfg(test)]
mod canvas_tests {
    use super::{Canvas, Color, NUM_COLOR_COMPONENTS};

    #[test]
    fn constructs_canvas_of_given_size() {
        let width = 64;
        let height = 32;

        let canvas = Canvas::new(width, height);

        assert_eq!(canvas.height, height);
        assert_eq!(canvas.width, width);
        assert_eq!(canvas.data.len(), width * height * NUM_COLOR_COMPONENTS);
    }

    #[test]
    fn writes_color_to_canvas_at_given_position() {
        let color = Color::new(1, 0, 0);

        let width = 64;
        let height = 32;
        let mut canvas = Canvas::new(width, height);

        let x = 10;
        let y = 15;

        canvas.set_pixel(x, y, &color);

        assert_eq!(canvas.data[y * width + x + 0], color.r);
        assert_eq!(canvas.data[y * width + x + 1], color.g);
        assert_eq!(canvas.data[y * width + x + 2], color.b);
        assert_eq!(canvas.data[y * width + x + 3], color.a);
    }
}
