use crate::vector::Vec3;

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
    time: f64
}

impl Ray {
    pub fn point_at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn direction(&self) -> Vec3 {
        self.direction.normalize().clone()
    }

    pub fn origin(&self) -> Vec3 {
        self.origin.clone()
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn new(origin: Vec3, direction: Vec3, time: f64) -> Ray {
        Ray { origin, direction, time }
    }
}