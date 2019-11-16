use crate::scene::{SceneItem, Sphere, MovingSphere};
use crate::ray::Ray;

pub struct Intersection {
    pub intersected: SceneItem,
    pub dist: f64
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> f64;
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> f64{
        let oc = ray.origin() - self.center();
        let a = ray.direction().dot(&ray.direction());
        let b = 2.0 * ray.direction().dot(&oc);
        let c = oc.dot(&oc) - self.radius() * self.radius();
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0. {
            return -1.;
        } else {
            return (-b - discriminant.sqrt()) / (2.0 * a)
        }
    }
}

impl Intersectable for MovingSphere {
    fn intersect(&self, ray: &Ray) -> f64 {
        let oc = ray.origin() - self.center(ray.time());
        let a = ray.direction().dot(&ray.direction());
        let b = 2.0 * ray.direction().dot(&oc);
        let c = oc.dot(&oc) - self.radius() * self.radius();
        let discriminant = b * b - 4.0 * a * c;
        if discriminant <= 0. {
            return -1.;
        } else {
            return (-b - discriminant.sqrt()) / (2.0 * a)
        }
    }
}

impl Intersectable for SceneItem {
    fn intersect(&self, ray: &Ray) -> f64{
        match self {
            SceneItem::Sphere(s) => s.intersect(ray),
            SceneItem::MovingSphere(s) => s.intersect(ray)
        }
    }
}
