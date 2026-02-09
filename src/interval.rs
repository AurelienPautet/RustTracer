use std::f64::{ NEG_INFINITY, INFINITY };

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn empty() -> Self {
        Self { min: INFINITY, max: NEG_INFINITY }
    }

    pub fn universe() -> Self {
        Self { min: NEG_INFINITY, max: INFINITY }
    }

    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, val: f64) -> bool {
        self.min <= val && val <= self.max
    }

    pub fn surrounds(&self, val: f64) -> bool {
        self.min < val && val < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        x.clamp(self.min, self.max)
    }
}
