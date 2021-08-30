use crate::hitable::{Hitable, HitRecord};
use crate::ray::Ray;
use glam::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32, rec:  &mut HitRecord ) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;

        if root < tmin || tmax < root {
            root = (-half_b + sqrtd) / a;

            if root < tmin || tmax < root {
                return false;
            }
            
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);

        true
    }
}
