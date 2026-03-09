use std::rc::Rc;

use crate::lib_core::{
    hittable::*, interval::Interval, material::Material, point::Point3, ray::Ray, vec::Vec3,
};

pub struct Sphere {
    center: Ray,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn stationary(static_center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center: Ray::new(static_center, Vec3::zero(), 0.0),
            radius,
            material,
        }
    }

    pub fn moving(
        center1: Point3,
        center2: Point3,
        radius: f64,
        material: Rc<dyn Material>,
    ) -> Self {
        Self {
            center: Ray::new(center1, center2 - center1, 0.0),
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let current_center = self.center.at(r.time());
        let oc = r.origin() - current_center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - current_center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = Some(self.material.clone());

        return true;
    }
}
