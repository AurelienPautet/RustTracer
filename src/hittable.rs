use std::sync::Arc;

use crate::{ interval::Interval, material::Material, ray::Ray, vec3::{ Point3, Vec3, dot } };

#[derive(Clone)]
pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: &'a Arc<dyn Material + Send + Sync>,
    pub t: f32,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        p: Point3,
        t: f32,
        ray: &Ray,
        mat: &'a Arc<dyn Material + Send + Sync>,
        outward_normal: Vec3
    ) -> Self {
        let front_face = dot(&ray.direction(), &outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };
        Self { p, normal, t, mat, front_face }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if let Some(r) = object.hit(&ray, Interval { min: ray_t.min, max: closest_so_far }) {
                closest_so_far = r.t;
                rec = Some(r);
            }
        }
        rec
    }
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}
