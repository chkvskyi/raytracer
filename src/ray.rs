use crate::vector::Vector3;

pub struct Ray {
    origin: Vector3,
    direction: Vector3
}

impl Ray {
    pub fn point_at(&self, t: f64) -> Vector3 {
        self.origin + t * self.direction
    }

    pub fn direction(&self) -> Vector3 {
        self.direction.clone()
    }

    pub fn new(origin: Vector3, dir: Vector3) -> Ray {
        Ray {
            origin: origin,
            direction: dir
        }
    }
}