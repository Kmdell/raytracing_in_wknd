use crate::{
    camera::{self, Camera},
    color::Color,
    hittable::{hittable_list::HittableList, HittableObject},
    material::MaterialType,
    simd_vec3::{Point3, Vec3},
    texture::TextureType,
};

pub fn quads() {
    let mut world = HittableList::default();

    // Materials
    let left_red = MaterialType::lambertion(TextureType::solid_color(&Color::new(1.0, 0.2, 0.2)));
    let back_green = MaterialType::lambertion(TextureType::solid_color(&Color::new(0.2, 1.0, 0.2)));
    let right_blue = MaterialType::lambertion(TextureType::solid_color(&Color::new(0.2, 0.2, 1.0)));
    let upper_orange =
        MaterialType::lambertion(TextureType::solid_color(&Color::new(1.0, 0.5, 0.0)));
    let lower_teal = MaterialType::lambertion(TextureType::solid_color(&Color::new(0.2, 0.8, 0.8)));

    // Objects
    world.add(HittableObject::quad(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    ));
    world.add(HittableObject::quad(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    ));
    world.add(HittableObject::quad(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    ));
    world.add(HittableObject::quad(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    ));
    world.add(HittableObject::quad(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    ));

    let mut camera = Camera::default();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1800;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.vfov = 80.0;
    camera.look_from = Point3::new(0.0, 0.0, 9.0);
    camera.look_at = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;

    camera.render(&world);
}
