use crate::ray::Ray;
use crate::vector::Vector3;
use rand::Rng;
use std::f64::consts::PI;

#[derive(Debug)]
pub struct Camera {
    lover_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    lens_radius: f64,
    pub origin: Vector3,
    u: Vector3,
    v: Vector3,
    w: Vector3
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(self.origin + offset, self.lover_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset)
    }

    pub fn new(position: Vector3, look_at: Vector3, up: Vector3, vfov: f64, aspect: f32, aperture: f64, focus_dist: f64) -> Camera {
        let theta: f64 = vfov * PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width = aspect as f64 * half_height;
        let w = (position - look_at).normalize();
        let u = up.cross(&w).normalize();
        let v = w.cross(&u);

        Camera {
            lover_left_corner: position - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w,
            horizontal: 2. * half_width * focus_dist * u,
            vertical: 2. * half_height * focus_dist * v,
            origin: position,
            lens_radius: aperture / 2.,
            u, v, w
        }
    }
}

fn random_in_unit_disk() -> Vector3 {
    let mut rng = rand::thread_rng();
    let mut p;
    while {
        p = 2. * Vector3::from_xyz(rng.gen(), rng.gen(), 0.) - Vector3::from_xyz(1., 1., 0.);
        p.dot(&p) >= 1.
    } {}
    return p
}