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

// pub fn random_unit_length_vector() -> Vec3 {
//    while true {
//        let p = Vec3::new(random_double, random_double);
//       
//        if p.len() {
//            unimplemented!();
//        }
//        
//        
//    } 
// }
