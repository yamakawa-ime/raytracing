use std::rc::Rc;

use crate::lib_core::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
};

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
    bbox: Aabb,
}

impl HittableList {
    pub fn default() -> Self {
        Self {
            objects: vec![],
            bbox: Aabb::default(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.bbox = Aabb::from_box(self.bbox, object.bounding_box());
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut hit_aything = false;
        let mut closest_so_far = ray_t.max();

        let mut temp_rec = HitRecord::default();
        for object in self.objects.iter() {
            if object.hit(r, Interval::new(ray_t.min(), closest_so_far), &mut temp_rec) {
                hit_aything = true;
                *rec = temp_rec.clone();
                closest_so_far = temp_rec.t;
            }
        }

        hit_aything
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
