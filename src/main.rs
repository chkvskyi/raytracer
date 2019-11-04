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

    let camera = Camera::new(
        Vector3::from_xyz(-2., 2., 1.),
        Vector3::from_xyz(0., 0., -1.),
        Vector3::from_xyz(0., 1., 0.),
        90., nx as f32 / ny as f32);
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
    let diff_center_mat = Material {
        color: Color::red(),
        albedo: 0.3,
        surface: Surface::Diffuse
    };
    let sphere = Sphere::new(Vector3::from_xyz(0., 0., -1.), 0.5, diff_center_mat);

    let diff_bottom_mat = Material {
        color: Color::green(),
        albedo: 0.3,
        surface: Surface::Diffuse
    };
    let sphere1 = Sphere::new(Vector3::from_xyz(0., -100.5, -1.), 100., diff_bottom_mat);

    let metall_mat = Material {
        color: Color::white(),
        albedo: 0.8,
        surface: Surface::Reflective {
            reflectivity: 0.5
        }
    };
    let right_sphere = Sphere::new(Vector3::from_xyz(1., 0., -1.), 0.5, metall_mat);

    let glass_mat = Material {
        color: Color::white(),
        albedo: 1.,
        surface: Surface::Refractive {
            index: 1.5
        }
    };
    let left_sphere = Sphere::new(Vector3::from_xyz(-1., 0., -1.), 0.5, glass_mat);
    let mut scene = Scene {
        items: Vec::new()
    };
    scene.items.push(sphere1);
    scene.items.push(sphere);
    scene.items.push(right_sphere);
    scene.items.push(left_sphere);

    scene
}
