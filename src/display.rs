use crate::defs::Real;
use crate::math::Color3;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    data: Vec<Color3>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            data: vec![Color3::default(); width * height],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: &Color3, samples_per_pixel: i32) {
        let mut red: Real = color[0];
        let mut green: Real = color[1];
        let mut blue: Real = color[2];

        let scale: Real = 1.0 / samples_per_pixel as Real;
        red = (scale * red).sqrt();
        green = (scale * green).sqrt();
        blue = (scale * blue).sqrt();

        let new_color = Color3::new(red, green, blue);

        self.data[(self.height as i32 - 1 - y as i32).abs() as usize * self.width + x] = new_color;
    }

    pub fn to_u8_vec(&self) -> Vec<u8> {
        let mut u8_vec = Vec::with_capacity(self.data.len() * 3);

        for color in self.data.iter() {
            u8_vec.push((Self::clamp(color[0], 0.0, 0.999) * 256.0) as u8);
            u8_vec.push((Self::clamp(color[1], 0.0, 0.999) * 256.0) as u8);
            u8_vec.push((Self::clamp(color[2], 0.0, 0.999) * 256.0) as u8);
        }

        u8_vec
    }

    fn clamp(val: Real, min: Real, max: Real) -> Real {
        if val < min {
            return min;
        }

        if val > max {
            return max;
        }

        val
    }
}
