use core::f64;

#[derive(Clone, Copy)]
pub struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn default() -> Self {
        Interval::empty()
    }

    pub fn from(a: Self, b: Self) -> Self {
        Self {
            min: if a.min() <= b.min() { a.min() } else { b.min() },
            max: if a.max() >= b.max() { a.max() } else { b.max() },
        }
    }

    pub fn empty() -> Self {
        Self {
            min: f64::INFINITY,
            max: -f64::INFINITY,
        }
    }

    pub fn universe() -> Self {
        Self {
            min: -f64::INFINITY,
            max: f64::INFINITY,
        }
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        return x;
    }

    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Self {
            min: self.min - padding,
            max: self.max + padding,
        }
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn max(&self) -> f64 {
        self.max
    }

    pub fn set_min(&mut self, val: f64) {
        self.min = val;
    }

    pub fn set_max(&mut self, val: f64) {
        self.max = val;
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }
}
