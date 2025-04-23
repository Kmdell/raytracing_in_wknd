use crate::{
    camera::Camera,
    color::Color,
    hittable::{hittable_list::HittableList, HittableObject},
    material::MaterialType,
    texture::TextureType,
    utility::{convert_to_linear, load_image},
    vec3::{Point3, Vec3},
};

pub fn earth() {
    let image = match load_image("earthmap.jpg") {
        Ok(e) => convert_to_linear(e),
        Err(e) => panic!("{:?}", e),
    };

    let earth_texture = TextureType::image(image);
    let earth_surface = MaterialType::lambertion(earth_texture);
    let globe = HittableObject::stationary_sphere(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface);

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1920;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;
    cam.background = Color::new(0.70, 0.80, 1.00);

    cam.vfov = 20.0;
    cam.look_from = Point3::new(0.0, 0.0, 12.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&HittableList::new(globe));
}
