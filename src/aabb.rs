use crate::scene::SceneItem;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::intersectable::{Intersectable, Intersection};
use rand::Rng;

use std::f64;
use std::cmp::Ordering;

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


pub struct BVH {
    // can has (left AND right) OR item
    pub bbox: AABB,
    left: Option<Box<BVH>>,
    right: Option<Box<BVH>>,
    item: Option<SceneItem>
}

impl BVH {
    pub fn new(items: &mut [SceneItem], time0: f64, time1: f64) -> BVH {
        let axis_ind = (3. * rand::thread_rng().gen::<f32>()) as u8;
        if axis_ind == 0 {
            items.sort_by(|a, b| a.bounding_box().min().x().partial_cmp(&b.bounding_box().min().x()).unwrap_or(Ordering::Equal));
        } else if axis_ind == 1 {
            items.sort_by(|a, b| a.bounding_box().min().y().partial_cmp(&b.bounding_box().min().y()).unwrap_or(Ordering::Equal));
        } else {
            items.sort_by(|a, b| a.bounding_box().min().z().partial_cmp(&b.bounding_box().min().z()).unwrap_or(Ordering::Equal));
        }

        if items.len() == 1 {
            return BVH {
                left: None,
                right: None,
                item: Some(items[0]),
                bbox: items[0].bounding_box()
            }
        } else {
            let middle = (items.len() / 2) as usize;

            let left = BVH::new(&mut items[0..middle], time0, time1);
            let right = BVH::new(&mut items[..middle], time0, time1);
            let l_bb = left.bounding_box();
            let r_bb = right.bounding_box();

            return BVH {
                left: Some(Box::new(left)),
                right: Some(Box::new(right)),
                bbox: surrounding_box(&l_bb, &r_bb),
                item: None
            }
        }
    }

    pub fn left(&self) -> Option<&BVH> { self.left.as_deref() }

    pub fn right(&self) -> Option<&BVH> { self.right.as_deref() }

    pub fn item(&self) -> Option<SceneItem> { self.item }

    pub fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        if self.bbox.intersect(&ray, t_min, t_max) {
            match self.left() {
                Some(left) => {
                    match self.right() {
                        Some(right) => {
                            let intersect_left = left.intersect(&ray, t_min, t_max);
                            let intersect_right = right.intersect(&ray, t_min, t_max);

                            match intersect_left {
                                Some(i_left) => {
                                    match intersect_right {
                                        Some(i_right) => {
                                            if i_left.dist < i_right.dist {
                                                return Some(i_left);
                                            } else {
                                                return Some(i_right);
                                            }
                                        },
                                        None => None
                                    }
                                },
                                None => {
                                    match intersect_right {
                                        Some(i_right) => {
                                            return Some(i_right);
                                        },
                                        None =>  None
                                    }
                                }
                            }
                        },
                        // BVH constructed wrong if some BVH has only left leaf
                        None => panic!("No right item")
                    }
                },
                None => {
                    match self.item {
                        Some(item) => {
                            let point = item.intersect(&ray);
                            if point > 0. {
                                return Some(Intersection {
                                    intersected: item,
                                    dist: point
                                })
                            } else {
                                return None;
                            }
                        },
                        None => None
                    }
                }
            }
        } else {
            return None
        }
    }
}

impl BoundingBox for BVH {
    fn bounding_box(&self) -> AABB {
        match self.left() {
            Some(left) => {
                match self.right() {
                    Some(right) => surrounding_box(&left.bbox, &right.bbox),
                    None => left.bbox
                }
            },
            None => {
                match self.item {
                    Some(item) => item.bounding_box(),
                    // if BVH not has leafs and not has item
                    None => panic!("Empty BVH")
                }
            }
        }
    }
}
