use std::{fmt::Display, ops::*};

use crate::rtweekend::{random_float, random_float_clamp};

const NEAR_ZERO: f32 = 1e-8;
pub type Point3 = Vec3;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Vec3 {
    pub e: [f32; 3],
}

impl Default for Vec3 {
    fn default() -> Self {
        Self { e: [0.0; 3] }
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { e: [x, y, z] }
    }

    pub fn random() -> Vec3 {
        Vec3::new(random_float(), random_float(), random_float())
    }

    pub fn random_clamp(min: f32, max: f32) -> Vec3 {
        Vec3::new(
            random_float_clamp(min, max),
            random_float_clamp(min, max),
            random_float_clamp(min, max),
        )
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }

    pub fn y(&self) -> f32 {
        self.e[1]
    }

    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.e.iter().map(|e| e * e).sum()
    }

    pub fn near_zero(&self) -> bool {
        self.e[0].abs() < NEAR_ZERO && self.e[1].abs() < NEAR_ZERO && self.e[2].abs() < NEAR_ZERO
    }

    #[inline]
    pub fn dot(&self, rhs: &Vec3) -> f32 {
        self.e
            .iter()
            .zip(rhs.e.iter())
            .map(|(e, rhs)| e * rhs)
            .sum()
    }

    #[inline]
    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3::new(
            self.e[1] * rhs.e[2] - self.e[2] * rhs.e[1],
            self.e[2] * rhs.e[0] - self.e[0] * rhs.e[2],
            self.e[0] * rhs.e[1] - self.e[1] * rhs.e[0],
        )
    }

    #[inline]
    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    #[inline]
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

    #[inline]
    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Self::random_clamp(-1.0, 1.0);
            let lensq = p.length_squared();
            if 1.0843e-19 < lensq && lensq <= 1.0 {
                break p / lensq.sqrt();
            }
        }
    }

    #[inline]
    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Self::random_unit_vector();
        // In the same hemisphere as the normal
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    #[inline]
    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        -2.0 * self.dot(normal) * normal + self
    }

    #[inline]
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

    fn neg(self) -> Self::Output {
        Self {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(mut self, rhs: Vec3) -> Self::Output {
        self += rhs;
        self
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.e[0] -= rhs.e[0];
        self.e[1] -= rhs.e[1];
        self.e[2] -= rhs.e[2];
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(mut self, rhs: Vec3) -> Self::Output {
        self -= rhs;
        self
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.e[0] *= rhs.e[0];
        self.e[1] *= rhs.e[1];
        self.e[2] *= rhs.e[2];
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(mut self, rhs: Vec3) -> Self::Output {
        self *= rhs;
        self
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(mut self, rhs: f32) -> Self::Output {
        self *= rhs;
        self
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self *= 1.0 / rhs;
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(mut self, rhs: f32) -> Self::Output {
        self /= rhs;
        self
    }
}

impl AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Vec3) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl Add<&Vec3> for Vec3 {
    type Output = Vec3;
    fn add(mut self, rhs: &Vec3) -> Self::Output {
        self += rhs;
        self
    }
}

impl SubAssign<&Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: &Vec3) {
        self.e[0] -= rhs.e[0];
        self.e[1] -= rhs.e[1];
        self.e[2] -= rhs.e[2];
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(mut self, rhs: &Vec3) -> Self::Output {
        self -= rhs;
        self
    }
}

impl MulAssign<&Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: &Vec3) {
        self.e[0] *= rhs.e[0];
        self.e[1] *= rhs.e[1];
        self.e[2] *= rhs.e[2];
    }
}

impl Mul<&Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(mut self, rhs: &Vec3) -> Self::Output {
        self *= rhs;
        self
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Mul<&Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            e: [rhs.e[0] * self, rhs.e[1] * self, rhs.e[2] * self],
        }
    }
}
