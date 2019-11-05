use image::{DynamicImage, GenericImage};
use rand::Rng;
use pbr::ProgressBar;
pub mod ray;
pub mod vector;
pub mod scene;
pub mod camera;
pub mod render;
pub mod color;

use vector::Vector3;
use scene::{Scene, Sphere, Material, Surface};
use camera::Camera;
use render::get_color;
use color::Color;

pub fn main() {
    let nx = 1920;
    let ny = 1080;
    let ns = 100;

    let mut rng = rand::thread_rng();
    let mut progress = ProgressBar::new(nx as u64);

    let camera_pos = Vector3::from_xyz(9., 6., 3.);
    let camera_look_at = Vector3::from_xyz(0., 0., 0.);
    let focus_dist = (camera_pos - camera_look_at).magn();
    let camera = Camera::new(
        camera_pos,
        camera_look_at,
        Vector3::from_xyz(0., 1., 0.),
        30., nx as f32 / ny as f32, 0.1, focus_dist);
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

    img.save("output2.png").unwrap();
    progress.finish_print("done");
}

fn init_scene() -> Scene {
    let diff_bottom_mat = Material {
        color: Color::green(),
        albedo: 0.3,
        surface: Surface::Diffuse
    };
    let sphere1 = Sphere::new(Vector3::from_xyz(0., -1000., -1.), 1000., diff_bottom_mat);

    let mut scene = Scene {
        items: Vec::new()
    };
    scene.items.push(sphere1);

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let mat_prob: f64 = rng.gen();
            let center = Vector3::from_xyz(a as f64 + 0.9 * rng.gen::<f64>(), 0.2, b as f64 + 0.9 * rng.gen::<f64>());

            if (center - Vector3::from_xyz(4., 0.2, 0.)).magn() > 0.9 {
                if mat_prob < 0.8 {
                    let diff_mat = Material {
                        color: Color::new(rng.gen(), rng.gen(), rng.gen()),
                        albedo: rng.gen(),
                        surface: Surface::Diffuse
                    };
                    let sphere = Sphere::new(center, 0.2, diff_mat);
                    scene.items.push(sphere);
                } else if mat_prob < 0.95 {
                    let metall_mat = Material {
                        color: Color::white(),
                        albedo: 0.8,
                        surface: Surface::Reflective {
                            reflectivity: rng.gen()
                        }
                    };
                    let right_sphere = Sphere::new(center, 0.2, metall_mat);
                    scene.items.push(right_sphere);
                } else {
                    let glass_mat = Material {
                        color: Color::white(),
                        albedo: 1.,
                        surface: Surface::Refractive {
                            index: 1.5
                        }
                    };
                    let left_sphere = Sphere::new(center, 0.2, glass_mat);
                    scene.items.push(left_sphere);
                }
            }
        }
    }
    scene.items.push(
        Sphere::new(Vector3::from_xyz(0., 1., 0.), 1., Material {
                color: Color::white(),
                albedo: 0.8,
                surface: Surface::Refractive {
                    index: 1.5
                }
            })
    );
    scene.items.push(
        Sphere::new(Vector3::from_xyz(-4., 1., 0.), 1., Material {
                color: Color::new(0.4, 0.2, 0.1),
                albedo: 0.8,
                surface: Surface::Diffuse
            })
    );
    scene.items.push(
        Sphere::new(Vector3::from_xyz(4., 1., 0.), 1., Material {
                color: Color::new(0.4, 0.2, 0.1),
                albedo: 0.8,
                surface: Surface::Reflective {
                    reflectivity: 0.
                }
            })
    );
    scene
}
