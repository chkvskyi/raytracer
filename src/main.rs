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
    if hit_sphere(Vector3::from_xyz(0., 0., -3.), 1., &r) {
        return Rgba([0, 150, 250, 0])
    }
    let unit = r.direction().normalize();
    let t = 0.5 * (unit.y + 1.0);
    let c = (1.0 - t) * Vector3::from_xyz(1.0, 1.0, 1.0) + t * Vector3::from_xyz(0.5, 0.7, 1.0);
    Rgba([(c.x * 255.0) as u8, (c.y * 255.0) as u8, (c.z * 255.0) as u8, 0])
}

fn hit_sphere(center: Vector3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(&ray.direction());
    let b = 2.0 * ray.direction().dot(&oc);
    let c = oc.dot(&oc) - radius * radius;

    b * b - 4.0 * a * c > 0.
}
