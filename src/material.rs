use glam::Vec3;

use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::utils::{random_double, random_unit_vector};

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

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = -uv.dot(n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -f32::sqrt(f32::abs(1.0 - r_out_perp.length_squared())) * n;

    r_out_perp + r_out_parallel
}

pub fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;

    r0 + (1.0 - r0) * f32::powf(1.0 - cosine, 5.0)
}

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
