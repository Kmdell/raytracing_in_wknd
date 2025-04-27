use crate::{
    camera::Camera,
    color::Color,
    hittable::{hittable_list::HittableList, HittableObject},
    material::MaterialType,
    simd_vec3::{Point3, Vec3},
    texture::TextureType,
    utility::{convert_to_linear, load_image, random_float_clamp},
};

pub fn final_scene(image_width: u32, samples_per_pixel: u32, max_depth: u32) {
    let mut boxes1 = HittableList::default();

    let ground = MaterialType::lambertion(Color::new(0.48, 0.83, 0.53).into());

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f32 * w;
            let z0 = -1000.0 + j as f32 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_float_clamp(1.0, 101.0);
            let z1 = z0 + w;

            if let HittableObject::HittableList(box1) = HittableObject::new_box(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            ) {
                let box1 = HittableObject::BvhNode(box1.into());
                boxes1.add(box1);
            }
        }
    }

    let mut world = HittableList::new(HittableObject::BvhNode(boxes1.into()));

    let light = MaterialType::diffuse_light(Color::new(7.0, 7.0, 7.0).into());
    world.add(HittableObject::quad(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light,
    ));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material = MaterialType::lambertion(Color::new(0.7, 0.3, 0.1).into());
    world.add(HittableObject::moving_sphere(
        center1,
        center2,
        50.0,
        sphere_material,
    ));

    world.add(HittableObject::stationary_sphere(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        MaterialType::dialectric(1.5),
    ));
    world.add(HittableObject::stationary_sphere(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        MaterialType::metal(Color::new(0.8, 0.8, 0.9), 1.0),
    ));

    let boundary = HittableObject::stationary_sphere(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        MaterialType::dialectric(1.5),
    );
    world.add(boundary.clone());
    world.add(HittableObject::constant_medium_color(
        boundary,
        0.2,
        &Color::new(0.2, 0.4, 0.9),
    ));
    let boundary = HittableObject::stationary_sphere(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        MaterialType::dialectric(1.5),
    );
    world.add(HittableObject::constant_medium_color(
        boundary,
        0.0001,
        &Color::new(1.0, 1.0, 1.0),
    ));

    let image = match load_image("earthmap.jpg") {
        Ok(e) => convert_to_linear(e),
        Err(e) => panic!("{:?}", e),
    };

    let emat = MaterialType::lambertion(TextureType::image(image));
    world.add(HittableObject::stationary_sphere(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    ));

    let pertext = TextureType::noise(0.2);
    world.add(HittableObject::stationary_sphere(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        MaterialType::lambertion(pertext),
    ));

    let mut boxes2 = HittableList::default();

    let white = MaterialType::lambertion(Color::new(0.73, 0.73, 0.73).into());
    for _ in 0..1000 {
        boxes2.add(HittableObject::stationary_sphere(
            Point3::random_clamp(0.0, 165.0),
            10.0,
            white.clone(),
        ));
    }

    let boxes2 = HittableObject::BvhNode(boxes2.into());

    world.add(HittableObject::translate(
        HittableObject::rotate_y(boxes2, 15.0),
        Vec3::new(-100.0, 270.0, 395.0),
    ));

    let world = HittableList::new(HittableObject::BvhNode(world.into()));

    let mut cam = Camera::default();

    cam.aspect_ratio = 1.0;
    cam.image_width = image_width;
    cam.samples_per_pixel = samples_per_pixel;
    cam.max_depth = max_depth;
    cam.background = Color::new(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.look_from = Point3::new(478.0, 278.0, -600.0);
    cam.look_at = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}
