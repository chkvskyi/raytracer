use crate::vector::Vec3;
use crate::ray::Ray;

#[derive(Copy, Clone, Debug)]
pub struct AABB {
    min: Vec3,
    max: Vec3
}

impl AABB {
    pub fn new(a: Vec3, b: Vec3) -> AABB {
        AABB { min: a, max: b }
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
    fn bounding_box(&self) -> AABB;
}

pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let small = Vec3::new(
        f64::min(box0.min().x(), box1.min().x()),
        f64::min(box0.min().y(), box1.min().y()),
        f64::min(box0.min().z(), box1.min().z())
    );
    let big = Vec3::new(
        f64::max(box0.max().x(), box1.max().x()),
        f64::max(box0.max().y(), box1.max().y()),
        f64::max(box0.max().z(), box1.max().z())
    );

    AABB::new(small, big)
}
