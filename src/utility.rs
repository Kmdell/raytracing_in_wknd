use std::{env::VarError, f32::consts::PI};

use image::{DynamicImage, GenericImageView, ImageBuffer, ImageError, Pixel, Rgb};
use rand::random_range;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator};

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

#[inline]
pub fn random_uint_clamp(min: usize, max: usize) -> usize {
    // returns a random integer in [min..max]
    random_range(min..=max)
}

pub fn load_image(filename: &str) -> Result<DynamicImage, ImageError> {
    let image_dir = match std::env::var("RTW_IMAGES") {
        Ok(image_dir) => Some(image_dir),
        Err(VarError::NotPresent) => None,
        Err(e) => panic!("{:?}", e),
    };
    if let Some(image_dir) = image_dir {
        return image::open(image_dir + filename);
    }

    if let Ok(image) = image::open(String::from("images/") + filename) {
        return Ok(image);
    } else if let Ok(image) = image::open(String::from("../images/") + filename) {
        return Ok(image);
    } else if let Ok(image) = image::open(String::from("../../images/") + filename) {
        return Ok(image);
    } else if let Ok(image) = image::open(String::from("../../../images/") + filename) {
        return Ok(image);
    } else if let Ok(image) = image::open(String::from("../../../../images/") + filename) {
        return Ok(image);
    } else if let Ok(image) = image::open(String::from("../../../../../images/") + filename) {
        return Ok(image);
    }
    image::open(String::from("../../../../../../images/") + filename)
}

pub fn convert_to_linear(image: DynamicImage) -> DynamicImage {
    let pixels: Vec<u8> = image
        .to_rgb32f()
        .pixels()
        .map(rgb_to_linear)
        .flatten()
        .collect();
    if let Some(img) =
        ImageBuffer::<Rgb<u8>, Vec<u8>>::from_vec(image.width(), image.height(), pixels)
    {
        img.into()
    } else {
        eprintln!("Failed to convert to linear");
        image
    }
}

fn rgb_to_linear(pixel: &Rgb<f32>) -> [u8; 3] {
    //println!("{}", pixel[0]);
    //println!("{}", pixel[1]);
    //println!("{}", pixel[2]);
    let linear_r = ((pixel[0]).powf(2.2) * 255.0) as u8;
    let linear_g = ((pixel[1]).powf(2.2) * 255.0) as u8;
    let linear_b = ((pixel[2]).powf(2.2) * 255.0) as u8;
    //println!("{}", linear_r);
    //println!("{}", linear_g);
    //println!("{}", linear_b);
    [linear_r, linear_g, linear_b]
}
