use crate::defs::Real;
use crate::math::{Color3, Ray, Vector3};
use crate::records::IntersectionRecord;

pub trait Material {
    fn scatter(&self, r: &Ray, intersection: &IntersectionRecord) -> MaterialInteraction;
}

pub struct MaterialInteraction {
    pub attenuation: Color3,
    pub scattered_ray: Ray,
    pub scattered: bool,
}

pub struct DefaultMaterial;

impl DefaultMaterial {
    pub fn new() -> DefaultMaterial {
        DefaultMaterial {}
    }
}

impl Material for DefaultMaterial {
    fn scatter(&self, _r: &Ray, _intersection: &IntersectionRecord) -> MaterialInteraction {
        MaterialInteraction {
            attenuation: Color3::new(0.0, 0.0, 0.0),
            scattered: false,
            scattered_ray: Ray::default(),
        }
    }
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
