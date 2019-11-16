#![feature(inner_deref)]
use image::{DynamicImage, GenericImage};
use rand::Rng;
use pbr::ProgressBar;
pub mod ray;
pub mod vector;
pub mod intersectable;
pub mod scene;
pub mod camera;
pub mod render;
pub mod color;
pub mod aabb;

use vector::Vec3;
use scene::{Scene, Sphere, MovingSphere, Material, Surface, SceneItem};
use camera::Camera;
use render::get_color;
use color::Color;

pub fn main() {
    let nx = 600;
    let ny = 400;
    let ns = 100;

    let mut rng = rand::thread_rng();
    let mut progress = ProgressBar::new(nx as u64);

    let camera_pos = Vec3::new(9., 6., 3.);
    let camera_look_at = Vec3::new(0., 0., 0.);
    let focus_dist = (camera_pos - camera_look_at).magn();
    let camera = Camera::new(
        camera_pos,
        camera_look_at,
        Vec3::new(0., 1., 0.),
        30., nx as f32 / ny as f32, 0.1, focus_dist, 0., 1.);
    let mut img = DynamicImage::new_rgb8(nx, ny);

    let scene = init_scene();

    for x in 0..nx {
        for y in 0..ny {
            let mut col = Color::black();
            for _s in 0..ns {
                let ru: f64 = rng.gen();
                let u = (x as f64 + ru) as f64 / nx as f64;
                let rv: f64 = rng.gen();
                let v = (ny as f64 - y as f64 + rv) as f64 / ny as f64;

                let r = camera.get_ray(u, v);
                col = col + get_color(&scene, &r, 1);
            }

            col = col / ns as f32;
            img.put_pixel(x, y, col.to_rgba());
        }
        progress.inc();
    }

    img.save("output.png").unwrap();
    progress.finish_print("done");
}

fn init_scene() -> Scene {
    let diff_bottom_mat = Material {
        color: Color::green(),
        albedo: 0.3,
        surface: Surface::Diffuse
    };
    let big_sphere = Sphere::new(Vec3::new(0., -1000., -1.), 1000., diff_bottom_mat);

    let mut items = Vec::new();
    items.push(SceneItem::Sphere(big_sphere));

    let mut rng = rand::thread_rng();
    // for a in -11..11 {
    //     for b in -11..11 {
    //         let mat_prob: f64 = rng.gen();
    //         let center = Vec3::new(a as f64 + 0.9 * rng.gen::<f64>(), 0.2, b as f64 + 0.9 * rng.gen::<f64>());

    //         if (center - Vec3::new(4., 0.2, 0.)).magn() > 0.9 {
    //             if mat_prob < 0.8 {
    //                 let diff_mat = Material {
    //                     color: Color::new(rng.gen(), rng.gen(), rng.gen()),
    //                     albedo: rng.gen(),
    //                     surface: Surface::Diffuse
    //                 };
    //                 let sphere = MovingSphere::new(center, Vec3::new(center.x(), center.y() + rng.gen::<f64>(), center.z()), 0., 1., 0.2, diff_mat);
    //                 items.push(SceneItem::MovingSphere(sphere));
    //             } else if mat_prob < 0.95 {
    //                 let metall_mat = Material {
    //                     color: Color::white(),
    //                     albedo: 0.8,
    //                     surface: Surface::Reflective {
    //                         reflectivity: rng.gen()
    //                     }
    //                 };
    //                 let metall_sphere = Sphere::new(center, 0.2, metall_mat);
    //                 items.push(SceneItem::Sphere(metall_sphere));
    //             } else {
    //                 let glass_mat = Material {
    //                     color: Color::white(),
    //                     albedo: 1.,
    //                     surface: Surface::Refractive {
    //                         index: 1.5
    //                     }
    //                 };
    //                 let left_sphere = Sphere::new(center, 0.2, glass_mat);
    //                 items.push(SceneItem::Sphere(left_sphere));
    //             }
    //         }
    //     }
    // }

    let s1 = Sphere::new(Vec3::new(0., 1., 0.), 1., Material {
                color: Color::white(),
                albedo: 0.8,
                surface: Surface::Refractive {
                    index: 1.5
                }
            });

    let s2 = Sphere::new(Vec3::new(-4., 1., 0.), 1., Material {
                color: Color::new(0.4, 0.2, 0.1),
                albedo: 0.8,
                surface: Surface::Diffuse
            });

    let s3 = Sphere::new(Vec3::new(4., 1., 0.), 1., Material {
        color: Color::new(0.4, 0.2, 0.1),
        albedo: 0.8,
        surface: Surface::Reflective {
            reflectivity: 0.
        }
    });
    items.push(SceneItem::Sphere(s1));
    items.push(SceneItem::Sphere(s2));
    items.push(SceneItem::Sphere(s3));

    Scene::new(items)
}
