use image::{DynamicImage, GenericImage, Rgba};
pub mod ray;
pub mod vector;

use ray::Ray;
use vector::Vector3;

pub fn main() {
    let nx = 200;
    let ny = 100;

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
    let unit = r.direction().normalize();
    let t = 0.5 * (unit.y + 1.0);
    let c = (1.0 - t) * Vector3::from_xyz(1.0, 1.0, 1.0) + t * Vector3::from_xyz(0.5, 0.7, 1.0);
    Rgba([(c.x * 255.0) as u8, (c.y * 255.0) as u8, (c.z * 255.0) as u8, 0])
}