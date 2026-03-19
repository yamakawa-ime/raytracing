use std::rc::Rc;

use crate::lib_core::{
    aabb::Aabb, interval::Interval, material::Material, point::Point3, ray::Ray, vec::Vec3,
};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Option<Rc<dyn Material>>,
    pub u: f64,
    pub v: f64,
}

impl HitRecord {
    pub fn default() -> Self {
        Self {
            p: Point3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            front_face: false,
            mat: None,
            u: 0.0,
            v: 0.0,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;

    fn bounding_box(&self) -> Aabb;
}
