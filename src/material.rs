use glam::Vec3;

use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::utils::random_unit_vector;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Scatter>;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

pub struct Scatter {
    pub attenuation: Vec3,
    pub scattered: Ray,
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

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * v.dot(*n) * *n
}

pub struct Metal {
    pub albedo: Vec3,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let reflected = reflect(&ray.direction.normalize(), &hit_record.normal);
        let scattered = Ray {
            origin: hit_record.p,
            direction: reflected,
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
