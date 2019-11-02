use image::{DynamicImage, GenericImage, Rgba};
use rand::Rng;
use pbr::ProgressBar;
pub mod ray;
pub mod vector;
pub mod scene;
pub mod camera;
pub mod render;

use vector::Vector3;
use scene::{Scene, Sphere, Material, Surface};
use camera::Camera;
use render::get_color;

pub fn main() {
    let nx = 2000;
    let ny = 1000;
    let ns = 100;

    let mut rng = rand::thread_rng();
    let mut progress = ProgressBar::new(nx as u64);

    let camera = Camera {
        lover_left_corner: Vector3::from_xyz(-2., -1., -1.),
        horizontal: Vector3::from_xyz(4., 0., 0.),
        vertical: Vector3::from_xyz(0., 2., 0.),
        origin: Vector3::zero()
    };

    let mut img = DynamicImage::new_rgb8(nx, ny);

    let scene = init_scene();

    for x in 0..nx {
        for y in 0..ny {
            let mut col = Vector3::zero();
            for _s in 0..ns {
                let ru: f64 = rng.gen();
                let u = (x as f64 + ru) as f64 / nx as f64;
                let rv: f64 = rng.gen();
                let v = (ny as f64 - y as f64 + rv) as f64 / ny as f64;

                let r = camera.get_ray(u, v);
                col = col + get_color(&scene, &r, 1);
            }

            col = col / ns as f64;
            col = Vector3::from_xyz(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
            col = 255. * col;
            img.put_pixel(x, y, Rgba([col.x as u8, col.y as u8, col.z as u8, 0]));
        }
        progress.inc();
    }

    img.save("tt1.png").unwrap();
    progress.finish_print("done");
}

fn init_scene() -> Scene {
    let diffuse_mat = Material {
        albedo: 0.5,
        surface: Surface::Diffuse
    };
    let metall_mat = Material {
        albedo: 1.,
        surface: Surface::Reflective {
            reflectivity: 0.5
        }
    };
    let metall_mat1 = Material {
        albedo: 1.,
        surface: Surface::Reflective {
            reflectivity: 0.1
        }
    };
    let sphere = Sphere::new(Vector3::from_xyz(0., 0., -1.), 0.5, diffuse_mat);
    let sphere1 = Sphere::new(Vector3::from_xyz(0., -100.5, -1.), 100., diffuse_mat);
    let metall_sphere = Sphere::new(Vector3::from_xyz(1., 0., -1.), 0.5, metall_mat);
    let metall_sphere1 = Sphere::new(Vector3::from_xyz(-1., 0., -1.), 0.5, metall_mat1);
    let mut scene = Scene {
        items: Vec::new()
    };
    scene.items.push(sphere);
    scene.items.push(metall_sphere);
    scene.items.push(metall_sphere1);
    scene.items.push(sphere1);

    scene
}
