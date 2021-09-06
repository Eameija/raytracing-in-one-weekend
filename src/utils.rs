use std::f32::consts::PI;
use glam::Vec3;
use rand::Rng;


pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn random_double() -> f32 {
    let mut rng = rand::thread_rng();
    
    rng.gen::<f32>()
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3::new(rng.gen::<f32>() * 2.0 - 1.0, rng.gen::<f32>() * 2.0 - 1.0, 0.0);
        if p.length_squared() < 1.0 {
            return p
        }
    }
}

pub fn random_unit_length_vector() -> Vec3 {
   loop {
       let p = Vec3::new(random_double(), random_double(), random_double());
       
       if p.length_squared() < 1.0 {
           break p;
       }
   } 
}

pub fn random_unit_vector() -> Vec3 {
    random_unit_length_vector().normalize()
}

pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_unit_length_vector();
    if in_unit_sphere.dot(*normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}
