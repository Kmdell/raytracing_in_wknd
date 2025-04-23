use crate::{
    camera::Camera,
    color::Color,
    hittable::{hittable_list::HittableList, HittableObject},
    material::MaterialType,
    texture::TextureType,
    vec3::Point3,
    vec3::Vec3,
};

pub fn simple_light() {
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
        MaterialType::lambertion(pertext),
    ));

    let difflight = MaterialType::diffuse_light(Color::new(4.0, 4.0, 4.0).into());

    world.add(HittableObject::stationary_sphere(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    ));

    world.add(HittableObject::quad(
        Point3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        difflight,
    ));

    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1800;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.0, 0.0, 0.0);

    cam.vfov = 20.0;
    cam.look_from = Point3::new(26.0, 3.0, 6.0);
    cam.look_at = Point3::new(0.0, 2.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    let world = HittableList::new(HittableObject::BvhNode(world.into()));

    cam.render(&world);
}
