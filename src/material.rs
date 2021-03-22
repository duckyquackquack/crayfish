use crate::defs::Real;
use crate::math::{Color3, Ray, Vector3};
use crate::records::IntersectionRecord;

use rand::Rng;

pub trait Material {
    fn scatter(&self, r: &Ray, intersection: &IntersectionRecord) -> MaterialInteraction;
}

pub struct MaterialInteraction {
    pub attenuation: Color3,
    pub scattered_ray: Ray,
    pub scattered: bool,
}

pub struct Lambertian {
    diffuse: Color3,
}

impl Lambertian {
    pub fn new(diffuse: Color3) -> Lambertian {
        Lambertian { diffuse }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, intersection: &IntersectionRecord) -> MaterialInteraction {
        let mut scatter_direction = intersection.normal + Vector3::random_in_unit_sphere();

        if scatter_direction.is_near_zero() {
            scatter_direction = intersection.normal;
        }

        MaterialInteraction {
            scattered: true,
            attenuation: self.diffuse,
            scattered_ray: Ray::new(intersection.point, scatter_direction),
        }
    }
}

pub struct Metal {
    diffuse: Color3,
    fuzz: Real,
}

impl Metal {
    pub fn new(diffuse: Color3, fuzz: Real) -> Metal {
        Metal {
            diffuse,
            fuzz: Self::clamp(fuzz, 0.0, 1.0),
        }
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

impl Material for Metal {
    fn scatter(&self, r: &Ray, intersection: &IntersectionRecord) -> MaterialInteraction {
        let reflected = r.direction.as_normal().reflect(&intersection.normal);

        MaterialInteraction {
            scattered_ray: Ray::new(
                intersection.point,
                reflected + (Vector3::random_in_unit_sphere() * self.fuzz),
            ),
            attenuation: self.diffuse,
            scattered: reflected.dot(&intersection.normal) > 0.0,
        }
    }
}

pub struct Dielectric {
    refraction_index: Real,
}

impl Dielectric {
    pub fn new(refraction_index: Real) -> Dielectric {
        Dielectric { refraction_index }
    }

    fn reflectance(cosine: Real, refraction_ratio: Real) -> Real {
        let mut r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, intersection: &IntersectionRecord) -> MaterialInteraction {
        let refraction_ratio = match intersection.front_face {
            true => (1.0 / self.refraction_index),
            false => self.refraction_index,
        };

        let direction = r.direction.as_normal();
        let cos_theta = Real::min(-direction.dot(&intersection.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let new_direction;
        let mut rng = rand::thread_rng();

        if cannot_refract
            || Self::reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0..1.0)
        {
            new_direction = direction.reflect(&intersection.normal);
        } else {
            new_direction = direction.refract(&intersection.normal, refraction_ratio);
        }

        MaterialInteraction {
            attenuation: Color3::new(1.0, 1.0, 1.0),
            scattered: true,
            scattered_ray: Ray::new(intersection.point, new_direction),
        }
    }
}
