use crate::camera::Camera;
use crate::defs::Real;
use crate::display::Canvas;
use crate::math::Color3;
use crate::math::Ray;
use crate::records::IntersectionRecord;
use crate::shapes::Shape;

use rand::{self, Rng};
use std::rc::Rc;

pub struct WorldRenderRequest {
    samples_per_pixel: i64,
    ray_max_depth: i64,
    ray_step: i64,
    width: usize,
    height: usize,
}

impl WorldRenderRequest {
    pub fn new(
        samples_per_pixel: i64,
        ray_max_depth: i64,
        ray_step: i64,
        width: usize,
        height: usize,
    ) -> WorldRenderRequest {
        WorldRenderRequest {
            samples_per_pixel,
            ray_max_depth,
            ray_step,
            width,
            height,
        }
    }
}

pub struct World {
    shapes: Vec<Rc<dyn Shape>>,
    camera: Camera,
}

impl World {
    pub fn new(camera: Camera) -> World {
        World {
            shapes: Vec::new(),
            camera,
        }
    }

    pub fn add_shape(&mut self, shape: Rc<dyn Shape>) {
        self.shapes.push(shape);
    }

    fn hit(&self, ray: &Ray, t_min: Real, t_max: Real) -> Option<IntersectionRecord> {
        let mut closest_intersection: Option<IntersectionRecord> = None;
        let mut closest_t: Real = t_max;

        for shape in self.shapes.iter() {
            let shape_intersection = shape.hit(&ray, t_min, closest_t);

            if let Some(ref intersection) = shape_intersection {
                closest_t = intersection.t;
                closest_intersection = shape_intersection;
            };
        }

        closest_intersection
    }

    fn color_at(&self, ray: &Ray, depth: i64) -> Color3 {
        if depth == 0 {
            return Color3::default();
        }

        let shape_intersection = self.hit(ray, 0.001, Real::INFINITY);

        if let Some(ref intersection) = shape_intersection {
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

    pub fn render(&self, render_request: WorldRenderRequest) -> Canvas {
        let mut canvas = Canvas::new(render_request.width, render_request.height);

        let mut rng = rand::thread_rng();

        for y in (0..render_request.height)
            .rev()
            .step_by(render_request.ray_step as usize)
        {
            println!("Remaining scanlines: {}", y);
            for x in (0..render_request.width).step_by(render_request.ray_step as usize) {
                let mut color = Color3::default();
                for _ in 0..render_request.samples_per_pixel {
                    let px: Real = (x as Real + rng.gen_range(0.0..1.0))
                        / (render_request.width as Real - 1.0);
                    let py: Real = (y as Real + rng.gen_range(0.0..1.0))
                        / (render_request.height as Real - 1.0);

                    let r = self.camera.get_ray(px, py);
                    color += self.color_at(&r, render_request.ray_max_depth);
                }

                canvas.set_pixel(x, y, &color, render_request.samples_per_pixel);
            }
        }

        canvas
    }
}
