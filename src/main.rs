#![feature(portable_simd)]
mod aabb;
mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod perlin;
mod ray;
mod scenes;
mod texture;
mod utility;

#[cfg(feature = "simd")]
pub mod simd_vec3;
#[cfg(feature = "simd")]
pub use simd_vec3 as vec3;
#[cfg(not(feature = "simd"))]
pub mod vec3;

fn main() {
    match 5 {
        2 => scenes::checkered_spheres(),
        3 => scenes::earth(),
        4 => scenes::perlin_spheres(),
        5 => scenes::quads(),
        _ => scenes::bouncing_spheres(),
    }
}
