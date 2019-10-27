use image::{DynamicImage, GenericImage, Rgba};
pub mod ray;
pub mod vector;

use ray::Ray;
use vector::Vector3;

pub fn main() {
    let nx = 600;
    let ny = 300;

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

    img.save("test.png").unwrap();
}

fn test_color(r: Ray) -> Rgba<u8> {
    let t =  hit_sphere(Vector3::from_xyz(0., 0., -2.), 0.5, &r);
    if t > 0. {
        let N = 0.5 * (r.point_at(t) - Vector3::from_xyz(0., 0., -2.)).normalize();
        let cN = 255. * Vector3::from_xyz(N.x + 0.5, N.y + 0.5, N.z + 0.5);
        return Rgba([cN.x as u8, cN.y as u8, cN.z as u8, 0]);
    }
    let unit = r.direction().normalize();
    let t = 0.5 * (unit.y + 1.0);
    let c = (1.0 - t) * Vector3::from_xyz(1.0, 1.0, 1.0) + t * Vector3::from_xyz(0.5, 0.7, 1.0);
    Rgba([(c.x * 255.0) as u8, (c.y * 255.0) as u8, (c.z * 255.0) as u8, 0])
}

fn hit_sphere(center: Vector3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(&ray.direction());
    let b = 2.0 * ray.direction().dot(&oc);
    let c = oc.dot(&oc) - radius * radius;
    let D = b * b - 4.0 * a * c;
    if D < 0. {
        return -1.;
    } else {
        return (-b - D.sqrt()) / (2.0 * a)
    }
}
