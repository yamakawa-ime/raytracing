use std::rc::Rc;

use raytracing::lib_core::{
    bvh::BvhNode,
    camera::Camera,
    color::Color,
    hittable::Hittable,
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Metal},
    point::Point3,
    rtweekend::{random_double, random_double_range},
    sphere::Sphere,
    texture::{CheckerTexture, ImageTexture},
    vec::Vec3,
};

fn main() {
    let mode = 3;
    match mode {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        _ => bouncing_spheres(),
    }
}

fn bouncing_spheres() {
    // World
    let mut world = HittableList::default();

    let checker = Rc::new(CheckerTexture::from_color(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    let ground_material = Rc::new(Lambertian::new(checker));
    world.add(Rc::new(Sphere::stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                f64::from(a) + 0.9 * random_double(),
                0.2,
                f64::from(b) + 0.9 * random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random_c() * Color::random_c();
                    let sphere_material = Rc::new(Lambertian::from(albedo));
                    let center2 = center + Vec3::new(0.0, random_double_range(0.0, 0.5), 0.0);
                    world.add(Rc::new(Sphere::moving(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range_c(0.5, 1.0);
                    let fuzzy = random_double_range(0.0, 0.5);
                    let sphere_material = Rc::new(Metal::new(albedo, fuzzy));
                    world.add(Rc::new(Sphere::stationary(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere::stationary(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::stationary(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::from(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::stationary(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::stationary(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let world = HittableList::new(Rc::new(BvhNode::from(&mut world)));
    // Camera
    let cam = Camera::new(
        16.0 / 9.0,
        400,
        10,
        10,
        20.0,
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.6,
        10.0,
    );
    cam.render(&world);
}

fn checkered_spheres() {
    let mut world = HittableList::default();

    let checker = Rc::new(CheckerTexture::from_color(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    world.add(Rc::new(Sphere::stationary(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Rc::new(Lambertian::new(checker.clone())),
    )));

    world.add(Rc::new(Sphere::stationary(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Rc::new(Lambertian::new(checker.clone())),
    )));

    let cam = Camera::new(
        16.0 / 9.0,
        400,
        10,
        50,
        20.0,
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
    );

    cam.render(&world);
}

fn earth() {
    let earth_texture = Rc::new(ImageTexture::new("raytracing/images/earthmap.jpg"));
    let earth_surface = Rc::new(Lambertian::new(earth_texture));
    let globe: Rc<dyn Hittable> = Rc::new(Sphere::stationary(
        Point3::new(0.0, 0.0, 0.0),
        2.0,
        earth_surface,
    ));

    let cam = Camera::new(
        16.0 / 9.0,
        400,
        10,
        50,
        20.0,
        Point3::new(0.0, 0.0, 12.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
    );

    cam.render(&HittableList::new(globe));
}
