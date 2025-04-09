use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
    simd::{cmp::SimdPartialOrd, num::SimdFloat, Simd},
};

use crate::rtweekend::{random_float, random_float_clamp};

pub type Point3 = Vec3;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Vec3 {
    e: Simd<f32, 3>,
}

impl Default for Vec3 {
    #[inline(always)]
    fn default() -> Self {
        Self {
            e: Simd::splat(0.0),
        }
    }
}

const NEAR_ZERO: f32 = 1e-8;

impl Vec3 {
    #[inline(always)]
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {
            e: Simd::from_array([x, y, z]),
        }
    }

    #[inline(always)]
    pub fn random() -> Vec3 {
        Vec3::new(random_float(), random_float(), random_float())
    }

    #[inline(always)]
    pub fn random_clamp(min: f32, max: f32) -> Vec3 {
        Vec3::new(
            random_float_clamp(min, max),
            random_float_clamp(min, max),
            random_float_clamp(min, max),
        )
    }

    #[inline(always)]
    pub fn x(&self) -> f32 {
        self[0]
    }

    #[inline(always)]
    pub fn y(&self) -> f32 {
        self[1]
    }

    #[inline(always)]
    pub fn z(&self) -> f32 {
        self[2]
    }

    #[inline(always)]
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    #[inline(always)]
    pub fn length_squared(&self) -> f32 {
        (self.e * self.e).reduce_sum()
    }

    #[inline(always)]
    pub fn near_zero(&self) -> bool {
        self.e.abs().simd_lt(Simd::splat(NEAR_ZERO)).all()
    }

    #[inline(always)]
    pub fn dot(&self, rhs: &Vec3) -> f32 {
        self.e.mul(rhs.e).reduce_sum()
    }

    #[inline(always)]
    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            e: (Simd::from_array([self[1], self[2], self[0]])
                * Simd::from_array([rhs[2], rhs[0], rhs[1]]))
                - (Simd::from_array([self[2], self[0], self[1]])
                    * Simd::from_array([rhs[1], rhs[2], rhs[0]])),
        }
    }

    #[inline(always)]
    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    #[inline(always)]
    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(
                random_float_clamp(-1.0, 1.0),
                random_float_clamp(-1.0, 1.0),
                0.0,
            );
            if p.length_squared() < 1.0 {
                break p;
            }
        }
    }

    #[inline(always)]
    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Self::random_clamp(-1.0, 1.0);
            let lensq = p.length_squared();
            if 1.0843e-19 < lensq && lensq <= 1.0 {
                break p / lensq.sqrt();
            }
        }
    }

    #[inline(always)]
    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Self::random_unit_vector();
        // In the same hemisphere as the normal
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    #[inline(always)]
    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        -2.0 * self.dot(normal) * normal + self
    }

    #[inline(always)]
    pub fn refract(&self, normal: &Vec3, etai_over_etat: f32) -> Vec3 {
        let temp = -*self;
        let costheta = temp.dot(normal).min(1.0);
        let r_out_perp = etai_over_etat * (costheta * normal + self);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * normal;

        r_out_perp + r_out_parallel
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Vec3 { e: self.e.neg() }
    }
}

impl Display for Vec3 {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self[0], self[1], self[2])
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl AddAssign<Vec3> for Vec3 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Vec3) {
        self.e.add_assign(rhs.e);
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn add(mut self, rhs: Vec3) -> Self::Output {
        self += rhs;
        self
    }
}

impl SubAssign<Vec3> for Vec3 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Vec3) {
        self.e.sub_assign(rhs.e);
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn sub(mut self, rhs: Vec3) -> Self::Output {
        self -= rhs;
        self
    }
}

impl MulAssign<Vec3> for Vec3 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Vec3) {
        self.e.mul_assign(rhs.e);
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn mul(mut self, rhs: Vec3) -> Self::Output {
        self *= rhs;
        self
    }
}

impl MulAssign<f32> for Vec3 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f32) {
        self.e.mul_assign(Simd::splat(rhs))
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn mul(mut self, rhs: f32) -> Self::Output {
        self *= rhs;
        self
    }
}

impl DivAssign<f32> for Vec3 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f32) {
        *self *= 1.0 / rhs;
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn div(mut self, rhs: f32) -> Self::Output {
        self /= rhs;
        self
    }
}

impl AddAssign<&Vec3> for Vec3 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: &Vec3) {
        self.e.add_assign(rhs.e);
    }
}

impl Add<&Vec3> for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn add(mut self, rhs: &Vec3) -> Self::Output {
        self += rhs;
        self
    }
}

impl SubAssign<&Vec3> for Vec3 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: &Vec3) {
        self.e.sub_assign(rhs.e);
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn sub(mut self, rhs: &Vec3) -> Self::Output {
        self -= rhs;
        self
    }
}

impl MulAssign<&Vec3> for Vec3 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: &Vec3) {
        self.e.mul_assign(rhs.e);
    }
}

impl Mul<&Vec3> for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn mul(mut self, rhs: &Vec3) -> Self::Output {
        self *= rhs;
        self
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    #[inline(always)]
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Mul<&Vec3> for f32 {
    type Output = Vec3;
    #[inline(always)]
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            e: Simd::splat(self).mul(rhs.e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_vec() {
        let vec = Vec3::default();
        assert_eq!(vec.e.to_array(), [0.0, 0.0, 0.0]);
    }

    #[test]
    fn new_vec() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(vec.e.to_array(), [1.0, 2.0, 3.0]);
    }
}
