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

    pub fn set_pixel(&mut self, x: usize, y: usize, color: &Color3, samples_per_pixel: i64) {
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

    pub fn to_u32_vec(&self) -> Vec<u32> {
        let mut u32_vec = Vec::with_capacity(self.data.len() * 3);

        for color in self.data.iter() {
            let r = (Self::clamp(color[0], 0.0, 0.999) * 256.0) as u8;
            let g = (Self::clamp(color[1], 0.0, 0.999) * 256.0) as u8;
            let b = (Self::clamp(color[2], 0.0, 0.999) * 256.0) as u8;

            u32_vec.push(Self::from_u8_rgb(r, g, b));
        }

        u32_vec
    }

    fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
        let (r, g, b) = (r as u32, g as u32, b as u32);
        (r << 16) | (g << 8) | b
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
