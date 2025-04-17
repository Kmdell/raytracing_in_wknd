use std::f32::INFINITY;

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub const EMPTY: Interval = Interval {
        min: INFINITY,
        max: -INFINITY,
    };

    pub const UNIVERSE: Interval = Interval {
        min: -INFINITY,
        max: INFINITY,
    };

    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn from_intervals(a: &Interval, b: &Interval) -> Interval {
        // Create the interval tightly enclosing the two input intervals

        Interval {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
    }

    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f32) -> f32 {
        x.clamp(self.min, self.max)
    }

    pub fn expands(&self, delta: f32) -> Interval {
        let padding = delta / 2.0;
        Interval::new(self.min - padding, self.max + padding)
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self::EMPTY
    }
}
