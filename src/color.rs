use crate::interval::Interval;

use super::vec3::Vec3;
pub type Color = Vec3;

const INTENSITY: Interval = Interval {
    min: 0.0,
    max: 0.999,
};

impl Color {
    fn linear_to_gamma(linear_component: f32) -> f32 {
        if linear_component > 0.0 {
            linear_component.sqrt()
        } else {
            0.0
        }
    }

    pub fn to_color(&self) -> String {
        let r = self.x();
        let g = self.y();
        let b = self.z();

        // Apply a linear to gamma transform for gama 2
        let r = Color::linear_to_gamma(r);
        let g = Color::linear_to_gamma(g);
        let b = Color::linear_to_gamma(b);

        // Translate the [0, 1] component values to byte range [0, 255]
        let r = (INTENSITY.clamp(r) * 256.0) as u8;
        let g = (INTENSITY.clamp(g) * 256.0) as u8;
        let b = (INTENSITY.clamp(b) * 256.0) as u8;

        // Write out pixel
        format!("{r} {g} {b}\n")
    }
}
