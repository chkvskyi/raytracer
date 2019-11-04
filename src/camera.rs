use crate::ray::Ray;
use crate::vector::Vector3;
use std::f64::consts::PI;

#[derive(Debug)]
pub struct Camera {
    pub lover_left_corner: Vector3,
    pub horizontal: Vector3,
    pub vertical: Vector3,
    pub origin: Vector3
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lover_left_corner + u * self.horizontal + v * self.vertical - self.origin)
    }

    pub fn new(position: Vector3, look_at: Vector3, up: Vector3, vfov: f64, aspect: f32) -> Camera {
        let theta: f64 = vfov * PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width = aspect as f64 * half_height;
        let w = (position - look_at).normalize();
        let u = up.cross(&w).normalize();
        let v = w.cross(&u);

        Camera {
            lover_left_corner: position - half_width * u - half_height * v - w,
            horizontal: 2. * half_width * u,
            vertical: 2. * half_height * v,
            origin: position
        }
    }
}