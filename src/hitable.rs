use crate::ray::Ray;
use glam::Vec3;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
}

pub struct HitableList {
    pub objects: Vec<Box<dyn Hitable>>,
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        let mut best_hit = None;
        let mut closest_so_far = tmax;

        for object in self.objects.iter() {
            if let Some(hit) = object.hit(ray, tmin, closest_so_far) {
                closest_so_far = hit.t;
                best_hit = Some(hit);
            }
        }

        best_hit
    }
}
