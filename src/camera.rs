use std::{f32::INFINITY, fs::File, io::Write, sync::Arc};

use indicatif::ParallelProgressIterator;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    color::Color,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    utility::{degree_to_radians, random_float},
    vec3::{Point3, Vec3},
};

const FILENAME: &str = "image.ppm";

pub struct Camera {
    // Ratio of image width over mage height
    pub aspect_ratio: f32,
    // Rendered image width in pixel count
    pub image_width: u32,
    // Count of random samples per pixel
    pub samples_per_pixel: u32,
    // Maximum numbers of ray bounces into scene
    pub max_depth: u32,
    // Scene background color
    pub background: Color,

    // Vertical view angle (field of view)
    pub vfov: f32,
    // Point camera is looking from
    pub look_from: Point3,
    // Point camera is looking at
    pub look_at: Point3,
    // Camera relative up direction
    pub vup: Vec3,

    // Variation angle of rays through each pixel
    pub defocus_angle: f32,
    // Distance from camera lookfrom point to plane of perfect focus
    pub focus_dist: f32,

    // Rendered image height
    image_height: u32,
    // Color scale factor for sum of pixel samples
    pixel_sample_scale: f32,
    // Camera Center
    center: Point3,
    // Location of pixel 0, 0
    pixel00_loc: Point3,
    // Offset of pixel to the right
    pixel_delta_u: Vec3,
    // Offset of pixel bollow
    pixel_delta_v: Vec3,
    // Camera frame basis vectors
    u: Vec3,
    v: Vec3,
    w: Vec3,
    // Defocus disk horizontal radius
    defocus_disk_u: Vec3,
    // Defocus disk vertical radius
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn render(&mut self, world: &impl Hittable) {
        self.initialize();
        // Dont care if it crashes
        let mut file = File::options()
            .write(true)
            .truncate(true)
            .open(FILENAME)
            .unwrap();

        let _ = file.write_fmt(format_args!(
            "P3\n{} {}\n255\n",
            self.image_width, self.image_height
        ));

        let arc = Arc::new(world);

        let start = std::time::Instant::now();
        let bytes: Vec<_> = (0..self.image_height)
            .collect::<Vec<_>>()
            .par_iter()
            .progress()
            .map(|j| {
                let world = arc.clone();

                let mut bytes = vec![];
                for i in 0..self.image_width {
                    let mut pixel_color = Color::default();

                    for _sample in 0..self.samples_per_pixel {
                        let ray = self.get_ray(i, *j);
                        pixel_color += self.ray_color(&ray, self.max_depth, *world.as_ref());
                    }
                    bytes.extend_from_slice(
                        format!(
                            "{}",
                            (self.pixel_sample_scale as f32 * pixel_color).to_color()
                        )
                        .as_bytes(),
                    );
                }
                bytes
            })
            .collect();
        let end = std::time::Instant::now();

        bytes
            .iter()
            .for_each(|bytes| file.write_all(bytes).unwrap());
        println!("Time taken to run: {}", (end - start).as_secs());
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f32 / self.aspect_ratio) as u32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };
        self.samples_per_pixel = if self.samples_per_pixel < 1 {
            1
        } else {
            self.samples_per_pixel
        };

        self.pixel_sample_scale = 1.0 / self.samples_per_pixel as f32;

        self.center = self.look_from;

        // Determine Viewport dimensions
        let theta = degree_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f32 / self.image_height as f32);

        // Calculate the u, v, w unit basis vectors for the camera coordinate frame.
        self.w = (self.look_from - self.look_at).unit_vector();
        self.u = self.vup.cross(&self.w).unit_vector();
        self.v = self.w.cross(&self.u);

        // Calculate the vectors across the horizontal and down the vertical edges of the viewport
        let viewport_u = viewport_width * self.u; // Vector across the horizontal edge
        let viewport_v = viewport_height * -self.v; // Vector down the vertical edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        // Calculate the position of the upper left pixel
        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // Calculate the camera defocus disk basis vector
        let defocus_radius = self.focus_dist * degree_to_radians(self.defocus_angle / 2.0).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        // Construct a camera Ray originating from the defocus disk and directed at randomly sampled
        // point around the pixel location i, j

        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f32 + offset.x()) * self.pixel_delta_u)
            + ((j as f32 + offset.y()) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_float();

        Ray::new(&ray_origin, &ray_direction, ray_time)
    }

    fn sample_square() -> Vec3 {
        // Returns the vector to a random point in [-0.5, -0.5]-[0.5, 0.5] unit square.
        Vec3::new(random_float() - 0.5, random_float() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        // Returns a random point on the defocus disk
        let p = Vec3::random_in_unit_disk();
        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }

    fn ray_color(&self, r: &Ray, depth: u32, world: &impl Hittable) -> Color {
        // If we hit the max ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::default();
        }

        let mut rec = HitRecord::default();

        // If the ray hits nothing, it returns the background color.
        if !world.hit(r, &Interval::new(0.001, INFINITY), &mut rec) {
            return self.background;
        }

        let mut scattered = Ray::default();
        let mut attenuation = Color::default();

        let color_from_emission = rec.mat.emitted(rec.u, rec.v, &rec.p);

        if !rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return color_from_emission;
        }

        let color_from_scatter = attenuation * self.ray_color(&scattered, depth - 1, world);

        color_from_scatter
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            background: Color::default(),
            vfov: 90.0,
            look_from: Point3::default(),
            look_at: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            image_height: 0,
            pixel_sample_scale: 0.0,
            center: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta_u: Point3::default(),
            pixel_delta_v: Point3::default(),
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
            defocus_disk_u: Vec3::default(),
            defocus_disk_v: Vec3::default(),
        }
    }
}
