use crate::vector::Vec3;
use crate::ray::Ray;

// use std::f64::{min, max};

fn ffmin(a: f64, b: f64) -> f64 { a.min(b) }
fn ffmax(a: f64, b: f64) -> f64 { a.max(b) }

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
            let min = self.min;
            let max = self.max;
            let t0: f64 = ffmin(min.get_ind(a) - ray.origin().get_ind(a) / ray.direction().get_ind(a),
                                max.get_ind(a) - ray.origin().get_ind(a) / ray.direction().get_ind(a));
            let t1: f64 = ffmax(min.get_ind(a) - ray.origin().get_ind(a) / ray.direction().get_ind(a),
                                max.get_ind(a) - ray.origin().get_ind(a) / ray.direction().get_ind(a));
            let tmin = ffmax(t0, tmin);
            let tmax = ffmin(t1, tmax);
            if tmax <= tmin {
                return false
            }
        }

        true
    }
}

pub trait BoundingBox {
    // TODO
}