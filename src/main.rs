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
mod simd_vec3;
#[cfg(feature = "simd")]
use simd_vec3 as vec3;
#[cfg(not(feature = "simd"))]
pub mod vec3;

fn main() {
    match 9 {
        1 => scenes::bouncing_spheres(),
        2 => scenes::checkered_spheres(),
        3 => scenes::earth(),
        4 => scenes::perlin_spheres(),
        5 => scenes::quads(),
        6 => scenes::simple_light(),
        7 => scenes::cornell_box(),
        8 => scenes::cornell_smoke(),
        9 => scenes::final_scene(800, 10000, 40),
        _ => scenes::final_scene(400, 250, 4),
    }
}
