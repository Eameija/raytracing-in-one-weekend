pub mod dielectric;
pub mod lambertian;
pub mod metal;

use glam::Vec3;

use crate::hitable::HitRecord;
use crate::ray::Ray;

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Scatter>;
}

pub struct Scatter {
    pub attenuation: Vec3,
    pub scattered: Ray,
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
