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
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32, rec: &mut HitRecord) -> bool;
}

pub struct HitableList {
    pub objects: Vec<Box<dyn Hitable>>,
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord {
            p: Vec3::ZERO,
            normal: Vec3::ZERO,
            t: 0.0,
            front_face: false,
        };

        let mut hit_anything = false;
        let mut closest_so_far = tmax;

        for object in self.objects.iter() {
            if object.hit(ray, tmin, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}
