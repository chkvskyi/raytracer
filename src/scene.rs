use crate::vector::Vector3;
use crate::ray::Ray;

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    center: Vector3,
    radius: f64,
    material: Material
}
impl Sphere {
    pub fn new(center: Vector3, radius: f64, material: Material) -> Sphere {
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

    pub fn center(&self) -> Vector3 {
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
pub enum Surface {
    Diffuse,
    Reflective,
    Refractive
}

#[derive(Copy, Clone, Debug)]
pub struct Material {
    pub albedo: f32,
    pub surface: Surface
}

pub struct Intersection {
    pub intersected: Sphere,
    pub dist: f64
}

pub struct Scene {
    pub items: Vec<Sphere>
}

impl Scene {
    pub fn trace(&self, r: &Ray) -> Option<Intersection> {
        let mut p: f64 = -1.;
        let mut i: Option<Sphere> = None;
        for item in self.items.iter() {
            let k = item.intersect(&r);

            if k > 0. {
                p = k;
                i = Some(*item);
                break;
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
