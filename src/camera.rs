use crate::ray::Ray;
use crate::vector::Vec3;
use rand::Rng;
use std::f64::consts::PI;

#[derive(Debug)]
pub struct Camera {
    lover_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f64,
    pub origin: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    t0: f64,
    t1: f64
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        let mut rng = rand::thread_rng();
        let random: f64 = rng.gen();
        let time = self.t0 + random * (self.t1 - self.t0);
        Ray::new(self.origin + offset, self.lover_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset, time)
    }

    pub fn new(position: Vec3, look_at: Vec3, up: Vec3, vfov: f64, aspect: f32, aperture: f64, focus_dist: f64, t0: f64, t1: f64) -> Camera {
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
            u, v, w, t0, t1
        }
    }
}

fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p;
    while {
        p = 2. * Vec3::new(rng.gen(), rng.gen(), 0.) - Vec3::new(1., 1., 0.);
        p.dot(&p) >= 1.
    } {}
    return p
}