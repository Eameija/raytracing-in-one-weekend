use glam::Vec3;
use rayon::prelude::*;

use crate::camera::Camera;
use crate::hitable::{Hitable, HitableList};
use crate::ray::Ray;
use crate::utils::random_double;

pub fn render(
    camera: &Camera,
    scene: &HitableList,
    width: u32,
    height: u32,
    samples: u32,
) -> Vec<u8> {
    let max_depth = 50;
    let image: Vec<u8> = (0..height)
        .into_par_iter()
        .rev()
        .flat_map(|y| {
            (0..width)
                .into_iter()
                .flat_map(|x| {
                    let color = (0..samples)
                        .map(|_| {
                            let u = ((x as f32) + random_double()) / ((width as f32) - 1.0);
                            let v = ((y as f32) + random_double()) / ((height as f32) - 1.0);

                            let ray = camera.get_ray(u, v);
                            ray_color(&ray, scene, max_depth)
                        })
                        .fold(Vec3::ZERO, |acc, x| acc + x);

                    let scale = 1.0 / (samples as f32);
                    let r = (color.x * scale).sqrt();
                    let g = (color.y * scale).sqrt();
                    let b = (color.z * scale).sqrt();

                    let r1 = (256.0 * r.clamp(0.0, 0.999)) as u8;
                    let g2 = (256.0 * g.clamp(0.0, 0.999)) as u8;
                    let b2 = (256.0 * b.clamp(0.0, 0.999)) as u8;
                    vec![r1, g2, b2, 255]
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<u8>>();

    image
}

fn ray_color(ray: &Ray, scene: &dyn Hitable, depth: u8) -> Vec3 {
    if depth == 0 {
        return Vec3::ZERO;
    }
    let color = scene
        .hit(ray, 0.001, f32::INFINITY)
        .and_then(|hit| hit.material.scatter(ray, &hit))
        .map(|scatter| scatter.attenuation * ray_color(&scatter.scattered, scene, depth - 1));

    if let Some(color) = color {
        return color;
    }

    let unit_direction = ray.direction.normalize();

    let t = 0.5 * (unit_direction.y + 1.0);

    (1.0 - t) * Vec3::ONE + t * Vec3::new(0.5, 0.7, 1.0)
}
