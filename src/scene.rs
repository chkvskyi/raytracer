use std::f64;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::color::Color;

// TODO: add Plane geometry

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

    pub fn intersect(&self, ray: &Ray) -> f64 {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(&ray.direction());
        let b = 2.0 * ray.direction().dot(&oc);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let D = b * b - 4.0 * a * c;
        if D < 0. {
            return -1.;
        } else {
            return (-b - D.sqrt()) / (2.0 * a)
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

    pub fn intersect(&self, ray: &Ray) -> f64 {
        let oc = ray.origin() - self.center(ray.time());
        let a = ray.direction().dot(&ray.direction());
        let b = 2.0 * ray.direction().dot(&oc);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let D = b * b - 4.0 * a * c;
        if D < 0. {
            return -1.;
        } else {
            return (-b - D.sqrt()) / (2.0 * a)
        }
    }

    pub fn material(&self) -> Material {
        self.material
    }

    pub fn center(&self, time: f64) -> Vec3 {
        self.center0 + ((time - self.t0) / (self.t1 - self.t0)) * (self.center1 - self.center0)
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

    pub fn intersect(&self, ray: &Ray) -> f64 {
        match self {
            SceneItem::Sphere(ref s) => s.intersect(&ray),
            SceneItem::MovingSphere(ref s) => s.intersect(&ray)
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

pub struct Intersection {
    pub intersected: SceneItem,
    pub dist: f64
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
}
