use crate::{
    camera::Camera,
    color::Color,
    hittable::{hittable_list::HittableList, HittableObject},
    material::MaterialType,
    vec3::{Point3, Vec3},
};

pub fn cornell_smoke() {
    let mut world = HittableList::default();

    let red = MaterialType::lambertion(Color::new(0.65, 0.05, 0.05).into());
    let white = MaterialType::lambertion(Color::new(0.73, 0.73, 0.73).into());
    let green = MaterialType::lambertion(Color::new(0.12, 0.45, 0.15).into());
    let light = MaterialType::diffuse_light(Color::new(7.0, 7.0, 7.0).into());

    world.add(HittableObject::quad(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    ));

    world.add(HittableObject::quad(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    ));

    world.add(HittableObject::quad(
        Point3::new(113.0, 554.0, 127.0),
        Vec3::new(333.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
        light,
    ));

    world.add(HittableObject::quad(
        Point3::new(0.0, 555.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    ));

    world.add(HittableObject::quad(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    ));

    world.add(HittableObject::quad(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    ));

    if let HittableObject::HittableList(box1) = HittableObject::new_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ) {
        let box1 = HittableObject::BvhNode(box1.into());
        //let box1 = HittableObject::HittableList(box1);
        let box1 = HittableObject::rotate_y(box1, 15.0);
        let box1 = HittableObject::translate(box1, Vec3::new(265.0, 0.0, 295.0));
        world.add(HittableObject::constant_medium_color(
            box1,
            0.01,
            &Color::new(0.0, 0.0, 0.0),
        ));
    }

    if let HittableObject::HittableList(box1) = HittableObject::new_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white,
    ) {
        let box1 = HittableObject::BvhNode(box1.into());
        //let box1 = HittableObject::HittableList(box1);
        let box1 = HittableObject::rotate_y(box1, -18.0);
        let box1 = HittableObject::translate(box1, Vec3::new(130.0, 0.0, 65.0));
        world.add(HittableObject::constant_medium_color(
            box1,
            0.01,
            &Color::new(1.0, 1.0, 1.0),
        ));
    }

    let world = HittableList::new(HittableObject::BvhNode(world.into()));

    let mut cam = Camera::default();

    cam.aspect_ratio = 1.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 200;
    cam.max_depth = 50;
    cam.background = Color::new(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.look_from = Point3::new(278.0, 278.0, -800.0);
    cam.look_at = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world)
}
