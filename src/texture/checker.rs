use std::sync::Arc;

use crate::color::Color;

use super::{Texture, TextureType};

#[derive(Clone)]
pub struct Checker {
    inv_scale: f32,
    even: Arc<TextureType>,
    odd: Arc<TextureType>,
}

impl Checker {
    pub fn new(scale: f32, even: TextureType, odd: TextureType) -> Checker {
        Checker {
            inv_scale: 1.0 / scale,
            even: Arc::new(even),
            odd: Arc::new(odd),
        }
    }

    pub fn from_colors(scale: f32, even: &Color, odd: &Color) -> Checker {
        Checker::new(
            scale,
            TextureType::solid_color(even),
            TextureType::solid_color(odd),
        )
    }
}

impl Texture for Checker {
    fn value(&self, u: f32, v: f32, p: &crate::simd_vec3::Point3) -> Color {
        let x_int = (p.x() * self.inv_scale).floor() as i32;
        let y_int = (p.y() * self.inv_scale).floor() as i32;
        let z_int = (p.z() * self.inv_scale).floor() as i32;

        let is_even = (x_int + y_int + z_int) % 2 == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}
