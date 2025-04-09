use std::f32::INFINITY;

pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    const EMPTY: Interval = Interval {
        min: INFINITY,
        max: -INFINITY,
    };

    const UNIVERSE: Interval = Interval {
        min: -INFINITY,
        max: INFINITY,
    };

    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
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
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: INFINITY,
            max: -INFINITY,
        }
    }
}
