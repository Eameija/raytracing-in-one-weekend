use crate::hitable::{Hitable, HitRecord};
use crate::material::Material;
use crate::ray::Ray;
use glam::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Box<dyn Material>,
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;

        if root < tmin || tmax < root {
            root = (-half_b + sqrtd) / a;

            if root < tmin || tmax < root {
                return None;
            }
            
        }

        let point = ray.at(root);
        let outward_normal = (point - self.center) / self.radius;
        let mut hit = HitRecord {
            t: root,
            p: point,
            normal: outward_normal,
            front_face: false,
            material: &*self.material,
        };

        hit.set_face_normal(ray, hit.normal);

        Some(hit)
    }
}
