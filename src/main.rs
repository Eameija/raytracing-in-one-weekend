use std::path::Path;
use std::{fs::File, io::BufWriter};

extern crate glam;

mod camera;
mod hitable;
mod ray;
mod sphere;
mod utils;
use hitable::{HitRecord, Hitable};
use glam::Vec3;
use ray::Ray;

use crate::camera::Camera;
use crate::utils::random_double;
use crate::{hitable::HitableList, sphere::Sphere};

fn main() {
    let width: u16 = 800;
    let aspect_ratio: f32 = 16.0 / 9.0;
    let height: u16 = (width as f32 / aspect_ratio) as u16;
    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = aspect_ratio * viewport_height;
    let focal_length = 1.0;
    let samples_per_pixel = 1;

    let origin = Vec3::ZERO;
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
    let path = Path::new(r"./image.png");
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, u32::from(width), u32::from(height));
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    let world = HitableList {
        objects: vec![
            Box::new(Sphere {
                center: Vec3::new(0.0, 0.0, -1.0),
                radius: 0.5,
            }),
            Box::new(Sphere {
                center: Vec3::new(1.0, 0.0, -1.0),
                radius: 0.5,
            }),
        ],
    };

    let camera = Camera::new();

    let mut pixels: Vec<u8> = vec![];

    for y in 0..height {
        for x in 0..width {
            let mut color = Vec3::ZERO;
            for s in 0..samples_per_pixel {
                let u: f32 = ((x as f32) + random_double()) / ((width as f32) - 1.0);
                let v: f32 = ((y as f32) + random_double()) / ((height as f32) - 1.0);

                let ray = camera.get_ray(u, v);
                color += ray_color(&ray, &world);
            }
            let scale = 1.0 / (samples_per_pixel as f32);
            let r = color.x * scale;
            let g = color.y * scale;
            let b = color.z * scale;

            pixels.push((256.0 * r.clamp(0.0, 0.999)) as u8);
            pixels.push((256.0 * g.clamp(0.0, 0.999)) as u8);
            pixels.push((256.0 * b.clamp(0.0, 0.999)) as u8);
            pixels.push(255);
        }
    }
    writer.write_image_data(&pixels).unwrap();
}

fn ray_color(ray: &Ray, world: &dyn Hitable) -> Vec3 {
    let mut hit_record = HitRecord {
        p: Vec3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 0.0),
        t: 0.0,
        front_face: false,
    };

    if world.hit(ray, 0.0, 1000000.0, &mut hit_record) {
        return 0.5 * (hit_record.normal + Vec3::ONE);
    }

    let unit_direction = ray.direction.clone().normalize();

    let t = 0.5 * (unit_direction[1] + 1.0);

    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}
