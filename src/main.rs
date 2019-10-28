use image::{DynamicImage, GenericImage, Rgba};
pub mod ray;
pub mod vector;
pub mod scene;

use ray::Ray;
use vector::Vector3;
use scene::{Sphere};

pub fn main() {
    let nx = 1920;
    let ny = 920;

    let lover_left_corner = Vector3::from_xyz(-2., -1., -1.);
    let horizontal = Vector3::from_xyz(4., 0., 0.);
    let vertical = Vector3::from_xyz(0., 2., 0.);
    let origin = Vector3::from_xyz(0., 0., 0.);

    let mut img = DynamicImage::new_rgb8(nx, ny);

    for x in 0..nx {
        for y in 0..ny {
            let u = x as f64 / nx as f64;
            let v = (ny - y) as f64 / ny as f64;

            let r = Ray::new(origin, lover_left_corner + u * horizontal + v * vertical);
            let c = test_color(r);
            img.put_pixel(x, y, c);
        }
    }

    img.save("normals.png").unwrap();
}

fn test_color(r: Ray) -> Rgba<u8> {
    let sphere = Sphere {
        center: Vector3::from_xyz(0., 0., -2.),
        radius: 0.5
    };
    let t =  sphere.intersect(&r);
    if t > 0. {
        let N = (r.point_at(t) - Vector3::from_xyz(0., 0., -2.)).normalize();
        let cN = 0.5 * 255. * Vector3::from_xyz(N.x + 1., N.y + 1., N.z + 1.);
        return Rgba([cN.x as u8, cN.y as u8, cN.z as u8, 0]);
    }
    let unit = r.direction().normalize();
    let t = 0.5 * (unit.y + 1.0);
    let c = (1.0 - t) * Vector3::from_xyz(1.0, 1.0, 1.0) + t * Vector3::from_xyz(0.5, 0.7, 1.0);
    Rgba([(c.x * 255.0) as u8, (c.y * 255.0) as u8, (c.z * 255.0) as u8, 0])
}
