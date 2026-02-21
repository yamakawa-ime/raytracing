use std::rc::Rc;

use crate::lib_core::hittable::{HitRecord, Hittable};

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn default() -> Self {
        Self { objects: vec![] }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl From<Rc<dyn Hittable>> for HittableList {
    fn from(value: Rc<dyn Hittable>) -> Self {
        Self {
            objects: vec![value],
        }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &super::ray::Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let mut hit_aything = false;
        let mut closest_so_far = ray_tmax;

        let mut temp_rec = HitRecord::default();
        for object in self.objects.iter() {
            if object.hit(r, ray_tmin, closest_so_far, &mut temp_rec) {
                hit_aything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_aything
    }
}
