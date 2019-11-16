use crate::vector::Vec3;
use crate::ray::Ray;

pub struct aabb {
    min: Vec3,
    max: Vec3
}

impl aabb {
    pub fn new(a: Vec3, b: Vec3) -> aabb {
        aabb { min: a, max: b }
    }

    pub fn min(&self) -> Vec3 { self.min }

    pub fn max(&self) -> Vec3 { self.max }

    pub fn intersect(&self, ray: &Ray, tmin: f64, tmax: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1. / ray.direction().get_ind(a);
            let mut t0 = (self.min.get_ind(a) - ray.origin().get_ind(a)) * inv_d;
            let mut t1 = (self.max.get_ind(a) - ray.origin().get_ind(a)) * inv_d;
            if inv_d < 0. {
                let s = t1;
                t1 = t0;
                t0 = s;
            }
            let t_min = match t0 > tmin {
                true => t0,
                false => tmin
            };
            let t_max = match t1 < tmax {
                true => t1,
                false => tmax
            };

            if t_max <= t_min {
                return false
            }
        }
        true
    }
}

pub trait BoundingBox {
    // TODO
}