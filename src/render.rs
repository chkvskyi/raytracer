use crate::scene::{Scene, Surface};
use crate::ray::Ray;
use crate::vector::Vec3;
use crate::color::Color;

use rand::Rng;

pub fn get_color(scene: &Scene, ray: &Ray, depth: u8) -> Color {
    if depth > 50 {
        return Color::black()
    }
    match scene.trace(&ray) {
        Some(intersection) => {
            let material = intersection.intersected.material();
            let normal = (ray.point_at(intersection.dist) - intersection.intersected.center()).normalize();

            match material.surface {
                Surface::Diffuse => {
                    let p = ray.point_at(intersection.dist);
                    let target = normal + p + random_unit_sphere();
                    return material.albedo * material.color * get_color(&scene, &Ray::new(p, target - p), depth + 1)
                },
                Surface::Reflective { reflectivity } => {
                    let reflected = reflect(ray.direction().normalize(), normal);
                    let scattered = Ray::new(intersection.intersected.center(), reflected + reflectivity as f64 * random_unit_sphere());
                    material.albedo as f32 * get_color(&scene, &scattered, depth + 1)
                }
                Surface::Refractive { index } => {
                    let outward_normal: Vec3;
                    let reflected = reflect(ray.direction().normalize(), normal);
                    let ni_over_nt: f32;
                    let cosine: f32;

                    if ray.direction().dot(&normal) > 0. {
                        outward_normal = - normal;
                        ni_over_nt = index;
                        cosine = index * ray.direction().dot(&normal) as f32 / ray.direction().magn() as f32;
                    } else {
                        outward_normal = normal;
                        ni_over_nt = 1. / index;
                        cosine = - ray.direction().dot(&normal) as f32 / ray.direction().magn() as f32;
                    }

                    match refract(ray.direction(), outward_normal, ni_over_nt) {
                        Some(refracted) => {
                            let prob = schlick(cosine, index);
                            let mut rng = rand::thread_rng();
                            let random: f32 = rng.gen();
                            if random < prob {
                                get_color(&scene, &Ray::new(intersection.intersected.center(), reflected), depth + 1)
                            } else {
                                get_color(&scene, &Ray::new(intersection.intersected.center(), refracted), depth + 1)
                            }
                        },
                        None => get_color(&scene, &Ray::new(intersection.intersected.center(), reflected), depth + 1)
                    }
                }
            }
        },
        None => {
            let unit = ray.direction().normalize();
            let t = 0.5 * (unit.y() + 1.0);
            let bg = Color::new(0.5, 0.7, 1.0);
            (1.0 - t) as f32 * Color::white() + t as f32 * bg
        }
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v + (- 2. * v.dot(&n) * n)
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.normalize();
    let dt = uv.dot(&n);
    let discriminant: f64 = 1. - ni_over_nt as f64 * ni_over_nt as f64 * (1. - dt * dt);
    match discriminant > 0. {
        true => Some(ni_over_nt as f64 * (uv - n * dt) - n * discriminant.sqrt()),
        false => None
    }
}

fn schlick(cosine: f32, ref_ind: f32) -> f32 {
    let mut r0 = (1. - ref_ind) / (1. + ref_ind);
    r0 = r0 * r0;
    r0 + (1. - r0) * (1. - cosine).powi(5)
}

fn random_unit_sphere() -> Vec3 {
    let mut p = Vec3::new(0., 0., 0.);
    let mut rng = rand::thread_rng();
    while p.magn() * p.magn() < 1. {
        p = Vec3::new(rng.gen(), rng.gen(), rng.gen());
    }
    p
}
