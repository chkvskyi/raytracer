use crate::vector::Vec3;

pub struct Ray {
    origin: Vec3,
    direction: Vec3
}

impl Ray {
    pub fn point_at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn direction(&self) -> Vec3 {
        self.direction.clone()
    }

    pub fn origin(&self) -> Vec3 {
        self.origin.clone()
    }

    pub fn new(origin: Vec3, dir: Vec3) -> Ray {
        Ray {
            origin: origin,
            direction: dir
        }
    }
}