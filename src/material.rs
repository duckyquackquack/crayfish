use crate::defs::Real;
use crate::math::{Color3, Ray, Vector3};
use crate::records::IntersectionRecord;

use rand::Rng;

pub enum Material {
    Lambertian { diffuse: Color3 },
    Metal { diffuse: Color3, fuzz: Real },
    Dielectric { refraction_index: Real },
}

pub trait Scatterer {
    fn scatter(&self, ray: &Ray, intersection: &IntersectionRecord) -> Option<MaterialInteraction>;
}

pub struct MaterialInteraction {
    pub attenuation: Color3,
    pub scattered_ray: Ray,
}

impl Scatterer for Material {
    fn scatter(&self, ray: &Ray, intersection: &IntersectionRecord) -> Option<MaterialInteraction> {
        match self {
            Material::Lambertian { diffuse } => lambertian(diffuse, ray, intersection),
            Material::Metal { diffuse, fuzz } => metal(diffuse, *fuzz, ray, intersection),
            Material::Dielectric { refraction_index } => {
                dielectric(*refraction_index, ray, intersection)
            }
        }
    }
}

fn lambertian(
    diffuse: &Color3,
    _ray: &Ray,
    intersection: &IntersectionRecord,
) -> Option<MaterialInteraction> {
    let mut scatter_direction = intersection.normal + Vector3::random_in_unit_sphere();

    if scatter_direction.is_near_zero() {
        scatter_direction = intersection.normal;
    }

    Some(MaterialInteraction {
        attenuation: *diffuse,
        scattered_ray: Ray::new(intersection.point, scatter_direction),
    })
}

fn metal(
    diffuse: &Color3,
    fuzz: Real,
    ray: &Ray,
    intersection: &IntersectionRecord,
) -> Option<MaterialInteraction> {
    let reflected = ray.direction.as_normal().reflect(&intersection.normal);

    if reflected.dot(&intersection.normal) <= 0.0 {
        return None;
    }

    Some(MaterialInteraction {
        scattered_ray: Ray::new(
            intersection.point,
            reflected + (Vector3::random_in_unit_sphere() * fuzz),
        ),
        attenuation: *diffuse,
    })
}

fn dielectric(
    refraction_index: Real,
    ray: &Ray,
    intersection: &IntersectionRecord,
) -> Option<MaterialInteraction> {
    let refraction_ratio = match intersection.front_face {
        true => (1.0 / refraction_index),
        false => refraction_index,
    };

    let direction = ray.direction.as_normal();
    let cos_theta = Real::min(-direction.dot(&intersection.normal), 1.0);
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

    let cannot_refract = refraction_ratio * sin_theta > 1.0;

    let new_direction;
    let mut rng = rand::thread_rng();

    if cannot_refract || {
        let mut r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cos_theta).powf(5.0)
    } > rng.gen_range(0.0..1.0)
    {
        new_direction = direction.reflect(&intersection.normal);
    } else {
        new_direction = direction.refract(&intersection.normal, refraction_ratio);
    }

    Some(MaterialInteraction {
        attenuation: Color3::new(1.0, 1.0, 1.0),
        scattered_ray: Ray::new(intersection.point, new_direction),
    })
}
