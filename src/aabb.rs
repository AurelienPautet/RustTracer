use crate::{ interval::Interval, vec3::Point3 };

#[derive(Clone)]
pub struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Aabb {
    pub fn new() -> Self {
        Aabb {
            x: Interval::new(f32::INFINITY, f32::NEG_INFINITY),
            y: Interval::new(f32::INFINITY, f32::NEG_INFINITY),
            z: Interval::new(f32::INFINITY, f32::NEG_INFINITY),
        }
    }

    pub fn new_from_interval(x: Interval, y: Interval, z: Interval) -> Self {
        Aabb { x, y, z }
    }

    pub fn new_from_points(a: &Point3, b: &Point3) -> Self {
        Aabb {
            x: if a.x() <= b.x() {
                Interval::new(a.x(), b.x())
            } else {
                Interval::new(b.x(), a.x())
            },
            y: if a.y() <= b.y() {
                Interval::new(a.y(), b.y())
            } else {
                Interval::new(b.y(), a.y())
            },
            z: if a.z() <= b.z() {
                Interval::new(a.z(), b.z())
            } else {
                Interval::new(b.z(), a.z())
            },
        }
    }

    pub fn new_from_aabbs(box0: &Self, box1: &Self) -> Self {
        Self {
            x: Interval::new(box0.x.min.min(box1.x.min), box0.x.max.max(box1.x.max)),
            y: Interval::new(box0.y.min.min(box1.y.min), box0.y.max.max(box1.y.max)),
            z: Interval::new(box0.z.min.min(box1.z.min), box0.z.max.max(box1.z.max)),
        }
    }

    pub fn axis_interval(&self, n: usize) -> &Interval {
        match n {
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }
    }

    #[inline]
    pub fn hit(&self, r: &crate::ray::Ray, ray_t: Interval) -> bool {
        let orig = r.origin();
        let dir = r.direction();
        let mut t_min = ray_t.min;
        let mut t_max = ray_t.max;

        // X
        let inv_d = 1.0 / dir.x();
        let mut t0 = (self.x.min - orig.x()) * inv_d;
        let mut t1 = (self.x.max - orig.x()) * inv_d;
        if inv_d < 0.0 {
            std::mem::swap(&mut t0, &mut t1);
        }
        t_min = t_min.max(t0);
        t_max = t_max.min(t1);
        if t_max <= t_min {
            return false;
        }

        // Y
        let inv_d = 1.0 / dir.y();
        let mut t0 = (self.y.min - orig.y()) * inv_d;
        let mut t1 = (self.y.max - orig.y()) * inv_d;
        if inv_d < 0.0 {
            std::mem::swap(&mut t0, &mut t1);
        }
        t_min = t_min.max(t0);
        t_max = t_max.min(t1);
        if t_max <= t_min {
            return false;
        }

        // Z
        let inv_d = 1.0 / dir.z();
        let mut t0 = (self.z.min - orig.z()) * inv_d;
        let mut t1 = (self.z.max - orig.z()) * inv_d;
        if inv_d < 0.0 {
            std::mem::swap(&mut t0, &mut t1);
        }
        t_min = t_min.max(t0);
        t_max = t_max.min(t1);
        if t_max <= t_min {
            return false;
        }

        true
    }

    pub fn longest_axis(&self) -> usize {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() { 0 } else { 2 }
        } else {
            if self.y.size() > self.z.size() { 1 } else { 2 }
        }
    }

    pub fn empty() -> Self {
        Aabb::new_from_interval(Interval::empty(), Interval::empty(), Interval::empty())
    }

    pub fn universe() -> Self {
        Aabb::new_from_interval(Interval::universe(), Interval::universe(), Interval::universe())
    }
}
