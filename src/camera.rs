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
        Ray::new(self.origin, self.lover_left_corner + u * self.horizontal + v * self.vertical)
    }

    pub fn new(vfov: f64, aspect: f32) -> Camera {
        let theta: f64 = vfov * PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width = aspect as f64 * half_height;
        Camera {
            lover_left_corner: Vector3::from_xyz(-half_width, -half_height, -1.),
            horizontal: Vector3::from_xyz(2. * half_width, 0., 0.),
            vertical: Vector3::from_xyz(0., 2. * half_height, 0.),
            origin: Vector3::zero()
        }
    }
}