use crate::{
    camera::Camera,
    color::Color,
    hittable::{hittable_list::HittableList, HittableObject},
    material::MaterialType,
    vec3::{Point3, Vec3},
};

pub fn cornell_box() {
    let mut world = HittableList::default();

    let red = MaterialType::lambertion(Color::new(0.65, 0.05, 0.05).into());
    let white = MaterialType::lambertion(Color::new(0.73, 0.73, 0.73).into());
    let green = MaterialType::lambertion(Color::new(0.12, 0.45, 0.15).into());
    let light = MaterialType::diffuse_light(Color::new(15.0, 15.0, 15.0).into());

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
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light,
    ));

    world.add(HittableObject::quad(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    ));

    world.add(HittableObject::quad(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
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
        let box1 = HittableObject::rotate_y(box1, 15.0);
        let box1 = HittableObject::translate(box1, Vec3::new(265.0, 0.0, 295.0));
        world.add(box1);
    }

    if let HittableObject::HittableList(box1) = HittableObject::new_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    ) {
        let box1 = HittableObject::BvhNode(box1.into());
        let box1 = HittableObject::rotate_y(box1, -18.0);
        let box1 = HittableObject::translate(box1, Vec3::new(130.0, 0.0, 65.0));
        world.add(box1);
    }

    //let world = HittableList::new(HittableObject::BvhNode(world.into()));

    let mut cam = Camera::default();

    cam.aspect_ratio = 1.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 200;
    cam.max_depth = 5;
    cam.background = Color::new(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.look_from = Point3::new(278.0, 278.0, -800.0);
    cam.look_at = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}
