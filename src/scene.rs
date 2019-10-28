use crate::vector::Vector3;
use crate::ray::Ray;

pub struct Sphere {
    pub center: Vector3,
    pub radius: f64
}
impl Sphere {
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
}

pub struct Scene {
    pub items: Vec<Sphere>
}

impl Scene {
    pub fn trace(&self, r: &Ray) -> f64 {
        let mut p: f64 = -1.;
        for item in self.items.iter() {
            let k = item.intersect(&r);
            if k > 0. {
                p = k;
            }
        }
        p
    }
}
