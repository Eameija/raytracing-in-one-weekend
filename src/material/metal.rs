use glam::Vec3;

use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::utils::random_unit_vector;

use super::{reflect, Material, Scatter};

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let reflected = reflect(&ray.direction.normalize(), &hit_record.normal);
        let scattered = Ray {
            origin: hit_record.p,
            direction: reflected + self.fuzz * random_unit_vector(),
        };

        if scattered.direction.dot(hit_record.normal) > 0.0 {
            Some(Scatter {
                attenuation: self.albedo,
                scattered,
            })
        } else {
            None
        }
    }
}
