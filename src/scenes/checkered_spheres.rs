use crate::{
    camera::Camera,
    color::Color,
    hittable::{hittable_list::HittableList, HittableObject},
    material::MaterialType,
    simd_vec3::{Point3, Vec3},
    texture::TextureType,
};

pub fn checkered_spheres() {
    let mut world = HittableList::default();

    let checkered = MaterialType::lambertion(TextureType::checker(
        0.32,
        &Color::new(0.2, 0.3, 0.1),
        &Color::new(0.9, 0.9, 0.9),
    ));

    world.add(HittableObject::stationary_sphere(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        checkered.clone(),
    ));

    world.add(HittableObject::stationary_sphere(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        checkered.clone(),
    ));

    //let world = HittableList::new(HittableObject::BvhNode(world.into()));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.look_from = Point3::new(13.0, 2.0, 13.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}
