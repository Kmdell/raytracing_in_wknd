use std::f32::consts::PI;

#[inline]
pub fn degree_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

#[inline]
pub fn random_float() -> f32 {
    // Returns a random real [0.0..1.0)
    rand::random_range(0.0..1.0)
}

#[inline]
pub fn random_float_clamp(min: f32, max: f32) -> f32 {
    // Returns a random real in [min..max)
    rand::random_range(min..max)
}
