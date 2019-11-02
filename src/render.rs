use crate::scene::{Scene, Surface};
use crate::ray::Ray;
use crate::vector::Vector3;

use rand::Rng;

pub fn get_color(scene: &Scene, ray: &Ray, depth: u8) -> Vector3 {
    if depth > 50 {
        return Vector3::zero()
    }
    match scene.trace(&ray) {
        Some(intersection) => {
            let material = intersection.intersected.material();
            let p = ray.point_at(intersection.dist);
            let normal = (ray.point_at(intersection.dist) - intersection.intersected.center()).normalize();

            match material.surface {
                Surface::Diffuse => {
                    let target = normal + p + random_unit_sphere();
                    return intersection.intersected.material().albedo as f64 * get_color(&scene, &Ray::new(p, target - p), depth + 1)
                },
                Surface::Reflective { reflectivity } => {
                    let dir = ray.direction().normalize();
                    let reflected = reflect(dir, normal);
                    let scattered = Ray::new(intersection.intersected.center(), reflected + reflectivity as f64 * random_unit_sphere());
                    material.albedo as f64 * get_color(&scene, &scattered, depth + 1)
                }
                _ => Vector3::zero()
            }

        },
        None => {
            let unit = ray.direction().normalize();
            let t = 0.5 * (unit.y + 1.0);
            (1.0 - t) * Vector3::from_xyz(1.0, 1.0, 1.0) + t * Vector3::from_xyz(0.5, 0.7, 1.0)
        }
    }
}

fn reflect(v: Vector3, n: Vector3) -> Vector3 {
    v + (- 2. * v.dot(&n) * n)
}

fn random_unit_sphere() -> Vector3 {
    let mut p = Vector3::from_xyz(0., 0., 0.);
    let mut rng = rand::thread_rng();
    while p.magn() * p.magn() < 1. {
        p = Vector3 {
            x: rng.gen(),
            y: rng.gen(),
            z: rng.gen()
        }
    }
    p
}
