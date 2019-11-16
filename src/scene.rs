use std::f64;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::color::Color;
use crate::aabb::aabb;
use crate::intersectable::{Intersection, Intersectable};

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Material
}
impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
            material: material
        }
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn material(&self) -> Material {
        self.material
    }

    pub fn bounding_box(&self) -> aabb {
        return aabb::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MovingSphere {
    center0: Vec3,
    center1: Vec3,
    t0: f64,
    t1: f64,
    radius: f64,
    material: Material,
}
impl MovingSphere {
    pub fn new(center0: Vec3, center1: Vec3, t0: f64, t1: f64, radius: f64, material: Material) -> MovingSphere {
        MovingSphere { center0, center1, t0, t1, radius, material }
    }

    pub fn material(&self) -> Material {
        self.material
    }

    pub fn center(&self, time: f64) -> Vec3 {
        self.center0 + ((time - self.t0) / (self.t1 - self.t0)) * (self.center1 - self.center0)
    }

    pub fn radius(&self) -> f64 { self.radius }

    pub fn bounding_box(&self) -> aabb {
        let start_bb = aabb::new(
            self.center(self.t0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(self.t0) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let finish_bb = aabb::new(
            self.center(self.t1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(self.t1) + Vec3::new(self.radius, self.radius, self.radius),
        );

        surrounding_box(&start_bb, &finish_bb)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum SceneItem {
    Sphere(Sphere),
    MovingSphere(MovingSphere)
}
impl SceneItem {
    pub fn material(&self) -> Material {
        match self {
            SceneItem::Sphere(ref s) => s.material(),
            SceneItem::MovingSphere(ref s) => s.material()
        }
    }

    pub fn center(&self, time: f64) -> Vec3 {
        match self {
            SceneItem::Sphere(ref s) => s.center(),
            SceneItem::MovingSphere(ref s) => s.center(time)
        }
    }

    pub fn bounding_box(&self) -> aabb {
        match self {
            SceneItem::Sphere(ref s) => s.bounding_box(),
            SceneItem::MovingSphere(ref s) => s.bounding_box()
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Surface {
    Diffuse,
    Reflective { reflectivity: f32 },
    Refractive { index: f32 }
}

#[derive(Copy, Clone, Debug)]
pub struct Material {
    pub color: Color,
    pub albedo: f32,
    pub surface: Surface
}

pub struct Scene {
    pub items: Vec<SceneItem>
}

impl Scene {
    pub fn trace(&self, r: &Ray) -> Option<Intersection> {
        let mut p: f64 = f64::MAX;
        let mut i: Option<SceneItem> = None;
        for item in self.items.iter() {
            let k = item.intersect(&r);
            if k > 0. && k < p {
                p = k;
                i = Some(*item);
            }
        }

        match i {
            Some(item) => Some(Intersection {
                intersected: item,
                dist: p
            }),
            None => None
        }
    }

    // // TODO: use trait
    pub fn bounding_box(&self, t0: f64, t1: f64, b: &aabb) -> aabb {
        let mut temp_box = self.items[0].bounding_box();

        for item in self.items.iter() {
            temp_box = surrounding_box(&item.bounding_box(), &temp_box);
        }
        return temp_box
    }
}

fn surrounding_box(box0: &aabb, box1: &aabb) -> aabb {
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

    aabb::new(small, big)
}
