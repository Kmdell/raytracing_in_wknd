use crate::{
    camera::Camera,
    color::Color,
    hittable::{hittable_list::HittableList, HittableObject},
    material::MaterialType,
    simd_vec3::{Point3, Vec3},
    texture::TextureType,
    utility::{random_float, random_float_clamp},
};

pub fn bouncing_spheres() {
    let mut world = HittableList::default();

    /* Main test scene
    let material_ground = MaterialType::lambertion(Color::new(0.8, 0.8, 0.0));
    let material_center = MaterialType::lambertion(Color::new(0.1, 0.2, 0.5));
    let material_left = MaterialType::dialectric(1.5);
    let material_bubble = MaterialType::dialectric(1.0 / 1.5);
    let material_right = MaterialType::metal(Color::new(0.8, 0.6, 0.2), 1.0);

    world.add(HittableObject::sphere(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.add(HittableObject::sphere(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    ));
    world.add(HittableObject::sphere(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    ));
    world.add(HittableObject::sphere(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    ));
    world.add(HittableObject::sphere(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ));
    */

    /* Two balls
        let r = (PI / 4.0).cos();

        let material_left = MaterialType::lambertion(Color::new(0., 0., 1.));
        let material_right = MaterialType::lambertion(Color::new(1., 0., 0.));

        world.add(HittableObject::sphere(
            Point3::new(-r, 0., -1.),
            r,
            material_left,
        ));
        world.add(HittableObject::sphere(
            Point3::new(r, 0., -1.),
            r,
            material_right,
        ));
    */

    let checker =
        TextureType::checker(0.32, &Color::new(0.2, 0.3, 0.1), &Color::new(0.9, 0.9, 0.9));

    let ground_material = MaterialType::lambertion(checker);
    world.add(HittableObject::stationary_sphere(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_float();
            let center = Point3::new(
                a as f32 + 0.9 * random_float(),
                0.2,
                b as f32 + 0.9 * random_float(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material =
                        MaterialType::lambertion(TextureType::solid_color(&albedo));
                    let center2 = center + Vec3::new(0.0, random_float_clamp(0.0, 0.5), 0.0);
                    world.add(HittableObject::moving_sphere(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    ));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Color::random_clamp(0.5, 1.0);
                    let fuzz = random_float_clamp(0.0, 0.5);
                    let sphere_material = MaterialType::metal(albedo, fuzz);
                    world.add(HittableObject::stationary_sphere(
                        center,
                        0.2,
                        sphere_material,
                    ));
                } else {
                    // Glass
                    let sphere_material = MaterialType::dialectric(1.5);
                    world.add(HittableObject::stationary_sphere(
                        center,
                        0.2,
                        sphere_material,
                    ));
                }
            }
        }
    }

    let material1 = MaterialType::dialectric(1.5);
    world.add(HittableObject::stationary_sphere(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    ));

    let material2 = MaterialType::lambertion(TextureType::solid_color(&Color::new(0.4, 0.2, 0.1)));
    world.add(HittableObject::stationary_sphere(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    ));

    let material3 = MaterialType::metal(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(HittableObject::stationary_sphere(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    ));

    let world = HittableList::new(HittableObject::BvhNode(world.into()));

    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.vfov = 20.0;
    camera.look_from = Point3::new(13.0, 2.0, 3.0);
    camera.look_at = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;

    camera.render(&world);
}
