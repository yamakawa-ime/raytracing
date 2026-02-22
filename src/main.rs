use std::rc::Rc;

use raytracing::lib_core::{
    camera::Camera, hittable_list::HittableList, point::Point3, sphere::Sphere,
};

fn main() {
    // World
    let mut world = HittableList::default();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let cam = Camera::new(16.0 / 9.0, 400, 5);
    cam.render(&world);
}
