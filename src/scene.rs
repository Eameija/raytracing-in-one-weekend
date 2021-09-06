use glam::Vec3;

use crate::hitable::HitableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::sphere::Sphere;
use crate::utils::random_double;

pub fn random_scene() -> HitableList {
    let mut scene = HitableList::new();
    let material_ground = Lambertian {
        albedo: Vec3::new(0.5, 0.5, 0.5),
    };

    let ground = Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        material: material_ground,
        radius: 1000.0,
    };

    scene.add(ground);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Vec3::new(
                (a as f32) + 0.9 * random_double(),
                0.2,
                (b as f32) + 0.9 * random_double(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3::new(random_double(), random_double(), random_double())
                        * Vec3::new(random_double(), random_double(), random_double());
                    scene.add(Sphere {
                        center,
                        radius: 0.2,
                        material: Lambertian { albedo },
                    })
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::new(
                        0.5 + (random_double() * 0.5),
                        0.5 + (random_double() * 0.5),
                        0.5 + (random_double() * 0.5),
                    );
                    let fuzz = random_double() * 0.5;
                    scene.add(Sphere {
                        center,
                        radius: 0.2,
                        material: Metal { albedo, fuzz },
                    })
                } else {
                    scene.add(Sphere {
                        center,
                        radius: 0.2,
                        material: Dielectric { ir: 1.5 },
                    })
                }
            }
        }
    }

    scene.add(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Dielectric { ir: 1.5 },
    });

    scene.add(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Lambertian {
            albedo: Vec3::new(0.4, 0.2, 0.1),
        },
    });

    scene.add(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        },
    });

    scene
}
