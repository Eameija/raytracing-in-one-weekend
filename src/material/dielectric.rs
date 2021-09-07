use glam::Vec3;

use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::utils::random_double;

use super::{reflect, reflectance, refract, Material, Scatter};

pub struct Dielectric {
    pub ir: f32,
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let attenuation = Vec3::ONE;
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = ray.direction.normalize();
        let cos_theta = f32::min(-unit_direction.dot(hit_record.normal), 1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);
        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction =
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_double() {
                reflect(&unit_direction, &hit_record.normal)
            } else {
                refract(unit_direction, hit_record.normal, refraction_ratio)
            };

        let scattered = Ray {
            origin: hit_record.p,
            direction,
        };

        Some(Scatter {
            attenuation,
            scattered,
        })
    }
}
