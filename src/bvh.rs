use crate::hittable::{ Hittable, HitRecord };
use crate::aabb::Aabb;
use crate::ray::Ray;
use crate::interval::Interval;

pub enum BvhNode {
    Leaf {
        object: Box<dyn Hittable>,
        bbox: Aabb,
    },
    Internal {
        left: Box<dyn Hittable>,
        right: Box<dyn Hittable>,
        bbox: Aabb,
    },
}

impl BvhNode {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> Self {
        Self::build(objects)
    }

    fn build(mut objects: Vec<Box<dyn Hittable>>) -> Self {
        let mut bbox = Aabb::empty();
        for obj in &objects {
            bbox = Aabb::new_from_aabbs(&bbox, obj.bounding_box());
        }

        let axis = bbox.longest_axis();
        let len = objects.len();

        if len == 1 {
            let object = objects.remove(0);
            BvhNode::Leaf { object, bbox }
        } else if len == 2 {
            let right = objects.remove(1);
            let left = objects.remove(0);
            BvhNode::Internal { left, right, bbox }
        } else {
            objects.sort_by(|a: &Box<dyn Hittable + 'static>, b| Self::box_compare(a.as_ref(), b.as_ref(), axis));
            let mid = len / 2;
            let right_objs = objects.split_off(mid);
            let left = Box::new(Self::build(objects)) as Box<dyn Hittable>;
            let right = Box::new(Self::build(right_objs)) as Box<dyn Hittable>;
            BvhNode::Internal { left, right, bbox }
        }
    }

    fn box_compare(a: &dyn Hittable, b: &dyn Hittable, axis: usize) -> std::cmp::Ordering {
        let a_min = a.bounding_box().axis_interval(axis).min;
        let b_min = b.bounding_box().axis_interval(axis).min;
        a_min.partial_cmp(&b_min).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        match self {
            BvhNode::Leaf { object, bbox } => {
                if !bbox.hit(r, ray_t) {
                    return None;
                }
                object.hit(r, ray_t)
            }
            BvhNode::Internal { left, right, bbox } => {
                if !bbox.hit(r, ray_t) {
                    return None;
                }

                if let Some(rec) = left.hit(r, ray_t) {
                    let t_max = rec.t;
                    if let Some(rec_right) = right.hit(r, Interval::new(ray_t.min, t_max)) {
                        if rec_right.t < rec.t {
                            return Some(rec_right);
                        }
                    }
                    Some(rec)
                } else {
                    right.hit(r, ray_t)
                }
            }
        }
    }

    fn bounding_box(&self) -> &Aabb {
        match self {
            BvhNode::Leaf { bbox, .. } => bbox,
            BvhNode::Internal { bbox, .. } => bbox,
        }
    }
}
