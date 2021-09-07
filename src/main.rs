mod camera;
mod hitable;
mod material;
mod output;
mod ray;
mod renderer;
mod scene;
mod sphere;
mod utils;

use crate::camera::Camera;
use crate::output::output;
use crate::renderer::render;
use crate::scene::random_scene;
use glam::Vec3;

fn main() {
    let width: u32 = 1200;
    let aspect_ratio: f32 = 3.0 / 2.0;
    let height: u32 = (width as f32 / aspect_ratio) as u32;
    let samples_per_pixel = 10;

    let scene = random_scene();
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);

    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.1,
        10.0,
    );

    let image = render(&camera, &scene, width, height, samples_per_pixel);
    output(image, width, height);
}
