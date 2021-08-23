use std::path::Path;
use std::{fs::File, io::BufWriter};

extern crate nalgebra;
mod hitable;
mod ray;
mod sphere;
mod utils;
use hitable::{HitRecord, Hitable};
use nalgebra::Vector3;
use ray::Ray;

use crate::{hitable::HitableList, sphere::Sphere};

fn main() {
    let width: u16 = 400;
    let aspect_ratio: f32 = 16.0 / 9.0;
    let height: u16 = (width as f32 / aspect_ratio) as u16;
    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vector3::new(0.0, 0.0, 0.0);
    let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, focal_length);
    let path = Path::new(r"./image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, u32::from(width), u32::from(height));
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    let world = HitableList {
        objects: vec![Box::new(Sphere {
            center: Vector3::new(0.0, 0.0, -1.0),
            radius: 0.5,
        }),
Box::new(Sphere {
            center: Vector3::new(1.0, 0.0, -1.0),
            radius: 0.5,
        })
        ],
    };

    let mut pixels: Vec<u8> = vec![];

    for y in 0..height {
        // println!("Remaining lines: {}", height - y);
        for x in 0..width {
            let xi = x as f32 / (width - 1) as f32;
            let yi = y as f32 / (height - 1) as f32;

            let ray = Ray {
                origin,
                direction: lower_left_corner + xi * horizontal + yi * vertical - origin,
            };
            let color = ray_color(&ray, &world);

            pixels.push((color.x * 255.0) as u8);
            pixels.push((color.y * 255.0) as u8);
            pixels.push((color.z * 255.0) as u8);
            pixels.push(255);
        }
    }
    writer.write_image_data(&pixels).unwrap();
}

fn ray_color(ray: &Ray, world: &dyn Hitable) -> Vector3<f32> {
    let mut hit_record = HitRecord {
        p: Vector3::new(0.0, 0.0, 0.0),
        normal: Vector3::new(0.0, 0.0, 0.0),
        t: 0.0,
        front_face: false,
    };

    if world.hit(ray, 0.0, 1000000.0, &mut hit_record) {
        return 0.5 * (hit_record.normal + Vector3::new(1.0, 1.0, 1.0));
    }

    let unit_direction = ray.direction.clone().normalize();

    let t = 0.5 * (unit_direction[1] + 1.0);

    (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
}
