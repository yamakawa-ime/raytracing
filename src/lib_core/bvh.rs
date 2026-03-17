use std::{cmp::Ordering, rc::Rc, usize};

use crate::lib_core::{
    aabb::Aabb, hittable::Hittable, hittable_list::HittableList, interval::Interval,
};

pub struct BvhNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn from(list: &mut HittableList) -> Self {
        let objects = list.objects_mut();
        Self::new(objects, 0, objects.len())
    }

    pub fn new(objects: &mut Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> Self {
        let mut bbox = Aabb::empty();
        for object_index in start..end {
            bbox = Aabb::from_box(bbox, objects[object_index].bounding_box());
        }

        let axis = bbox.longest_axis();

        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            _ => Self::box_z_compare,
        };

        let object_span = end - start;

        let (left, right) = match object_span {
            1 => {
                let left_obj = objects[start].clone();
                let right_obj = objects[start].clone();
                (left_obj, right_obj)
            }
            2 => {
                let left_obj = objects[start].clone();
                let right_obj = objects[start + 1].clone();
                (left_obj, right_obj)
            }
            _ => {
                objects[start..end].sort_by(comparator);
                let mid = start + object_span / 2;
                let left_obj: Rc<dyn Hittable> = Rc::new(BvhNode::new(objects, start, mid));
                let right_obj: Rc<dyn Hittable> = Rc::new(BvhNode::new(objects, mid, end));
                (left_obj, right_obj)
            }
        };
        Self { left, right, bbox }
    }

    fn box_x_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        let a_axis = a.bounding_box().axis_interval(0);
        let b_axis = b.bounding_box().axis_interval(0);
        a_axis.min().partial_cmp(&b_axis.min()).unwrap()
    }

    fn box_y_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        let a_axis = a.bounding_box().axis_interval(1);
        let b_axis = b.bounding_box().axis_interval(1);
        a_axis.min().partial_cmp(&b_axis.min()).unwrap()
    }

    fn box_z_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        let a_axis = a.bounding_box().axis_interval(2);
        let b_axis = b.bounding_box().axis_interval(2);
        a_axis.min().partial_cmp(&b_axis.min()).unwrap()
    }
}

impl Hittable for BvhNode {
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
    fn hit(
        &self,
        r: &super::ray::Ray,
        ray_t: super::interval::Interval,
        rec: &mut super::hittable::HitRecord,
    ) -> bool {
        if !self.bbox.hit(r, ray_t) {
            return false;
        }

        let hit_left = self.left.hit(r, ray_t, rec);

        let hit_right = self.right.hit(
            r,
            if hit_left {
                Interval::new(ray_t.min(), rec.t)
            } else {
                ray_t
            },
            rec,
        );

        hit_left || hit_right
    }
}
