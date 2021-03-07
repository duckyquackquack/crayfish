use crate::camera::Camera;
use crate::defs::Real;
use crate::display::Canvas;
use crate::math::Color3;
use crate::math::Ray;
use crate::records::IntersectionRecord;
use crate::shapes::Shape;

use rand::{self, Rng};
use std::rc::Rc;

pub struct World {
    shapes: Vec<Rc<dyn Shape>>,
    camera: Camera,
}

impl World {
    pub fn new() -> World {
        World {
            shapes: Vec::new(),
            camera: Camera::new(),
        }
    }

    pub fn add_shape(&mut self, shape: Rc<dyn Shape>) {
        self.shapes.push(shape);
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    fn hit(&self, ray: &Ray, t_min: Real, t_max: Real) -> IntersectionRecord {
        let mut closest_intersection = IntersectionRecord::default();
        let mut closest_t: Real = t_max;

        for shape in self.shapes.iter() {
            let shape_intersection = shape.hit(&ray, t_min, closest_t);

            if shape_intersection.hit {
                closest_t = closest_intersection.t;
                closest_intersection = shape_intersection;
            }
        }

        closest_intersection
    }

    fn color_at(&self, ray: &Ray, depth: u16) -> Color3 {
        if depth == 0 {
            return Color3::default();
        }

        let intersection = self.hit(ray, 0.001, 100000.0);
        if intersection.hit {
            let material_interaction = intersection.material.scatter(&ray, &intersection);

            if material_interaction.scattered {
                return material_interaction.attenuation
                    * self.color_at(&material_interaction.scattered_ray, depth - 1);
            }
            return Color3::new(0.0, 0.0, 0.0);
        }

        let blue = Color3::new(0.5, 0.7, 1.0);
        let white = Color3::new(1.0, 1.0, 1.0);

        let direction = ray.direction.as_normal();
        let interp: Real = 0.5 * (direction[1] + 1.0);

        white * (1.0 - interp) + (blue * interp)
    }

    pub fn render(&self, width: usize, height: usize) -> Canvas {
        let mut canvas = Canvas::new(width, height);
        let samples_per_pixel = 50;
        let max_depth = 50;

        let mut rng = rand::thread_rng();

        let step = 1;

        for y in (0..height).rev().step_by(step) {
            for x in (0..width).step_by(step) {
                let mut color = Color3::default();
                for _ in 0..samples_per_pixel {
                    let px: Real = (x as Real + rng.gen_range(0.0..1.0)) / (width as Real - 1.0);
                    let py: Real = (y as Real + rng.gen_range(0.0..1.0)) / (height as Real - 1.0);

                    let r = self.camera.get_ray(px, py);
                    color += self.color_at(&r, max_depth);
                }

                canvas.set_pixel(x, y, &color, samples_per_pixel);
            }
        }

        canvas
    }
}
