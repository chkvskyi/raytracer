use image::{DynamicImage, GenericImage, Rgba};
use rand::Rng;
pub mod ray;
pub mod vector;
pub mod scene;
pub mod camera;

use ray::Ray;
use vector::Vector3;
use scene::{Scene, Sphere};
use camera::Camera;

pub fn main() {
    let nx = 1920;
    let ny = 1080;
    let ns = 100;

    let mut rng = rand::thread_rng();

    let camera = Camera {
        lover_left_corner: Vector3::from_xyz(-2., -1., -1.),
        horizontal: Vector3::from_xyz(4., 0., 0.),
        vertical: Vector3::from_xyz(0., 2., 0.),
        origin: Vector3::from_xyz(0., 0., 0.)
    };

    let mut img = DynamicImage::new_rgb8(nx, ny);

    for x in 0..nx {
        for y in 0..ny {
            let mut col = [0., 0., 0., 0.];
            for s in 0..ns {
                let ru: f64 = rng.gen();
                let u = (x as f64 + ru) as f64 / nx as f64;
                let rv: f64 = rng.gen();
                let v = (ny as f64 - y as f64 + rv) as f64 / ny as f64;

                let r = camera.get_ray(u, v);
                let c = color(&r);
                col[0] += c[0];
                col[1] += c[1];
                col[2] += c[2];
            }

            img.put_pixel(x, y, Rgba([(255. * col[0] / ns as f64) as u8, (255. * col[1] / ns as f64) as u8, (255. * col[2] / ns as f64) as u8, 0]));
        }
    }

    img.save("normals.png").unwrap();
}

fn color(r: &Ray) -> [f64; 4] {
    let sphere = Sphere {
        center: Vector3::from_xyz(0., 0., -2.),
        radius: 0.5
    };
    let sphere1 = Sphere {
        center: Vector3::from_xyz(1., 1., -3.),
        radius: 1.
    };
    let mut scene = Scene {
        items: Vec::new()
    };
    scene.items.push(sphere);
    scene.items.push(sphere1);
    let t =  scene.trace(&r);
    if t > 0. {
        let N = (r.point_at(t) - Vector3::from_xyz(0., 0., -2.)).normalize();
        let cN = 0.5 * Vector3::from_xyz(N.x + 1., N.y + 1., N.z + 1.);
        return [cN.x, cN.y, cN.z, 0.];
    }
    let unit = r.direction().normalize();
    let t = 0.5 * (unit.y + 1.0);
    let c = (1.0 - t) * Vector3::from_xyz(1.0, 1.0, 1.0) + t * Vector3::from_xyz(0.5, 0.7, 1.0);
    [c.x, c.y, c.z, 0.]
}
