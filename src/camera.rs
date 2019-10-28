use crate::ray::Ray;
use crate::vector::Vector3;

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
}