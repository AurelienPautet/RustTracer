use crate::{ interval::Interval, ray::Ray, vec3::{ Point3, Vec3, dot } };

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, t: f64, ray: &Ray, outward_normal: Vec3) -> Self {
        let front_face = dot(&ray.direction(), &outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };
        Self { p, normal, t, front_face }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord>;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if let Some(r) = object.hit(ray, Interval { min: ray_t.min, max: closest_so_far }) {
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
