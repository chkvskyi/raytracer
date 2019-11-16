use crate::scene::SceneItem;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::intersectable::{Intersectable, Intersection};
use rand::Rng;

use std::f64;
use std::cmp::Ordering;
use std::mem;

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

    pub fn intersect(&self, ray: &Ray) -> bool {
        let invn_dir_x = 1. / ray.direction().x();
        let invn_dir_y = 1. / ray.direction().y();
        let invn_dir_z = 1. / ray.direction().z();

        let mut tx_min = (self.min.x() - ray.origin().x()) * invn_dir_x;
        let mut tx_max = (self.max.x() - ray.origin().x()) * invn_dir_x;
        if invn_dir_x < 0. {
            mem::swap(&mut tx_min, &mut tx_max);
        }
        let mut ty_min = (self.min.y() - ray.origin().y()) * invn_dir_y;
        let mut ty_max = (self.max.y() - ray.origin().y()) * invn_dir_y;
        if invn_dir_y < 0. {
            mem::swap(&mut ty_min, &mut ty_max);
        }

        if (tx_min > ty_max) || (ty_min > tx_max) {
            return false;
        }

        if ty_min > tx_min {
            tx_min = ty_min;
        }
        if ty_max < tx_max {
            tx_max = ty_max
        }

        let mut tz_min = (self.min.z() - ray.origin().z()) * invn_dir_z;
        let mut tz_max = (self.max.z() - ray.origin().z()) * invn_dir_z;
        if invn_dir_z < 0. {
            mem::swap(&mut tz_min, &mut tz_max);
        }

        if tx_min > tz_max || tz_min > tx_max {
            return false
        }
        // if tz_min > tx_min {
        //     tx_min = tz_min
        // }
        // if tz_max < tx_max {
        //     tx_max = tz_max
        // }

        return true
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
    pub fn new(items: &mut [SceneItem]) -> BVH {
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

            let left = BVH::new(&mut items[0..middle]);
            let right = BVH::new(&mut items[middle..]);
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

    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
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
            None => {
                if self.bbox.intersect(&ray) {
                    let l = self.left().unwrap();
                    let r = self.right().unwrap();

                    let intersect_left = l.intersect(&ray);
                    let intersect_right = r.intersect(&ray);

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
                                None => {
                                    return Some(i_left)
                                }
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
                } else {
                    return None;
                }
            }
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
