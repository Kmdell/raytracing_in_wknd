use crate::{
    simd_vec3::{Point3, Vec3},
    utility::{random_float, random_uint_clamp},
};

const POINT_COUNT: usize = 256;

#[derive(Clone)]
pub struct Perlin {
    randvec: [Vec3; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Perlin {
        let mut randvec = [Vec3::default(); POINT_COUNT];
        let mut perm_x = [0; POINT_COUNT];
        let mut perm_y = [0; POINT_COUNT];
        let mut perm_z = [0; POINT_COUNT];
        for i in 0..POINT_COUNT {
            randvec[i] = Vec3::random_clamp(-1.0, 1.0).unit_vector();
        }

        perlin_generate_perm(&mut perm_x);
        perlin_generate_perm(&mut perm_y);
        perlin_generate_perm(&mut perm_z);

        Perlin {
            randvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Point3) -> f32 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as isize;
        let j = p.y().floor() as isize;
        let k = p.z().floor() as isize;

        let mut c: [[[Vec3; 2]; 2]; 2] = [
            [
                [Vec3::default(), Vec3::default()],
                [Vec3::default(), Vec3::default()],
            ],
            [
                [Vec3::default(), Vec3::default()],
                [Vec3::default(), Vec3::default()],
            ],
        ];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let i = self.perm_x[((i + di as isize) & 255) as usize];
                    let j = self.perm_y[((j + dj as isize) & 255) as usize];
                    let k = self.perm_z[((k + dk as isize) & 255) as usize];
                    c[di][dj][dk] = self.randvec[i ^ j ^ k];
                }
            }
        }

        trilinear_interop(&c, u, v, w)
    }

    pub fn turb(&self, p: &Point3, depth: usize) -> f32 {
        let mut accum = 0.0;

        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }
}

fn perlin_generate_perm(p: &mut [usize]) {
    for i in 0..p.len() {
        p[i] = i;
    }

    permute(p);
}

fn permute(p: &mut [usize]) {
    for i in (0..p.len()).rev() {
        let target = random_uint_clamp(0, i);
        let tmp = p[i];
        p[i] = p[target];
        p[target] = tmp;
    }
}

fn trilinear_interop(c: &[[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let mut accum = 0.0;

    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight_v = Vec3::new(u - i as f32, v - j as f32, w - k as f32);

                let di = (i as f32 * uu) + (1.0 - i as f32) * (1.0 - uu);
                let dj = (j as f32 * vv) + (1.0 - j as f32) * (1.0 - vv);
                let dk = (k as f32 * ww) + (1.0 - k as f32) * (1.0 - ww);
                accum += di * dj * dk * c[i][j][k].dot(&weight_v);
            }
        }
    }

    accum
}
