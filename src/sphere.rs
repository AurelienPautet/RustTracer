use std::sync::Arc;

use crate::{
    hittable::{ HitRecord, Hittable },
    interval::Interval,
    material::Material,
    vec3::{ Point3, dot },
};

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Arc<dyn Material + Send + Sync>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Arc<dyn Material + Send + Sync>) -> Self {
        Sphere { center, radius: radius.max(0.0), mat }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: crate::ray::Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin();
        let a = ray.direction().length_squared();
        let h = dot(&ray.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root: f64 = (h - sqrtd) / a;

        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let p = ray.at(t);
        let outward_normal = (p - self.center) / self.radius;
        Some(HitRecord::new(p, t, &ray, &self.mat, outward_normal))
    }
}
