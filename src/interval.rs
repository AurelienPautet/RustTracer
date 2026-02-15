use std::f32::{ NEG_INFINITY, INFINITY };

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn empty() -> Self {
        Self { min: INFINITY, max: NEG_INFINITY }
    }

    pub fn universe() -> Self {
        Self { min: NEG_INFINITY, max: INFINITY }
    }

    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn new_from_intervals(a: Self, b: Self) -> Self {
        Self {
            min: if a.min <= b.min {
                a.min
            } else {
                b.min
            },
            max: if a.max >= b.max {
                a.max
            } else {
                b.max
            },
        }
    }

    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    pub fn contains(&self, val: f32) -> bool {
        self.min <= val && val <= self.max
    }

    pub fn surrounds(&self, val: f32) -> bool {
        self.min < val && val < self.max
    }

    pub fn clamp(&self, x: f32) -> f32 {
        x.clamp(self.min, self.max)
    }

    pub fn expand(&self, delta: f32) -> Self {
        let padding = delta / 2.0;
        Interval { min: self.min - padding, max: self.max + padding }
    }
}
