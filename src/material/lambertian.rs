use glam::Vec3;

use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::utils::random_unit_vector;

use super::{Material, Scatter};

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let scatter_direction = hit_record.normal + random_unit_vector();

        let scattered = Ray {
            origin: hit_record.p,
            direction: scatter_direction,
        };
        let attenuation = self.albedo;
        Some(Scatter {
            attenuation,
            scattered,
        })
    }
}
