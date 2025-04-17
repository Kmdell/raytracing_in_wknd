use crate::{
    camera::Camera,
    hittable::{hittable_list::HittableList, HittableObject},
    material::MaterialType,
    simd_vec3::{Point3, Vec3},
    texture::TextureType,
};

pub fn perlin_spheres() {
    let mut world = HittableList::default();

    let pertext = TextureType::noise(4.0);
    world.add(HittableObject::stationary_sphere(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        MaterialType::lambertion(pertext.clone()),
    ));
    world.add(HittableObject::stationary_sphere(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        MaterialType::lambertion(pertext.clone()),
    ));

    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.vfov = 20.0;
    camera.look_from = Point3::new(13.0, 2.0, 3.0);
    camera.look_at = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;

    camera.render(&world);
}
