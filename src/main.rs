use image::{DynamicImage, GenericImage, Rgba};
use rand::Rng;
use pbr::ProgressBar;
pub mod ray;
pub mod vector;
pub mod scene;
pub mod camera;

use ray::Ray;
use vector::Vector3;
use scene::{Scene, Sphere, Material, Surface};
use camera::Camera;

pub fn main() {
    let nx = 400;
    let ny = 200;
    let ns = 100;

    let mut rng = rand::thread_rng();

    let camera = Camera {
        lover_left_corner: Vector3::from_xyz(-2., -1., -1.),
        horizontal: Vector3::from_xyz(4., 0., 0.),
        vertical: Vector3::from_xyz(0., 2., 0.),
        origin: Vector3::zero()
    };

    let mut img = DynamicImage::new_rgb8(nx, ny);

    let mut progress = ProgressBar::new(nx as u64);

    for x in 0..nx {
        for y in 0..ny {
            let mut col = Vector3::zero();
            for _s in 0..ns {
                let ru: f64 = rng.gen();
                let u = (x as f64 + ru) as f64 / nx as f64;
                let rv: f64 = rng.gen();
                let v = (ny as f64 - y as f64 + rv) as f64 / ny as f64;

                let r = camera.get_ray(u, v);
                col = col + color(&r);
            }

            col = col / ns as f64;
            col = Vector3::from_xyz(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
            col = 255. * col;
            img.put_pixel(x, y, Rgba([col.x as u8, col.y as u8, col.z as u8, 0]));
        }
        progress.inc();
    }

    img.save("tt.png").unwrap();
    progress.finish_print("done");
}

fn color(r: &Ray) -> Vector3 {
    let mat = Material {
        albedo: 0.5,
        surface: Surface::Diffuse
    };
    let sphere = Sphere::new(Vector3::from_xyz(0., 0., -2.), 0.5, mat);
    let sphere1 = Sphere::new(Vector3::from_xyz(0., -100.5, -1.), 100., mat);
    let mut scene = Scene {
        items: Vec::new()
    };
    scene.items.push(sphere);
    scene.items.push(sphere1);

    match scene.trace(&r) {
        Some(intersection) => {
            let p = r.point_at(intersection.dist);
            let N = (r.point_at(intersection.dist) - intersection.intersected.center()).normalize();
            let target = N + p + random_unit_sphere();
            let cN = 0.5 * Vector3::from_xyz(N.x + 1., N.y + 1., N.z + 1.);
            return intersection.intersected.material().albedo as f64 * color(&Ray::new(p, target - p))
        },
        None => {
            let unit = r.direction().normalize();
            let t = 0.5 * (unit.y + 1.0);
            (1.0 - t) * Vector3::from_xyz(1.0, 1.0, 1.0) + t * Vector3::from_xyz(0.5, 0.7, 1.0)
        }
    }
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
