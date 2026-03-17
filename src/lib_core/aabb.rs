use crate::lib_core::{interval::Interval, point::Point3, ray::Ray};

#[derive(Clone, Copy)]
pub struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Aabb {
    pub fn empty() -> Self {
        Self {
            x: Interval::empty(),
            y: Interval::empty(),
            z: Interval::empty(),
        }
    }

    pub fn universe() -> Self {
        Self {
            x: Interval::universe(),
            y: Interval::universe(),
            z: Interval::universe(),
        }
    }

    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_point(a: Point3, b: Point3) -> Self {
        let x = if a[0] <= b[0] {
            Interval::new(a[0], b[0])
        } else {
            Interval::new(b[0], a[0])
        };
        let y = if a[1] <= b[1] {
            Interval::new(a[1], b[1])
        } else {
            Interval::new(b[1], a[1])
        };
        let z = if a[2] <= b[2] {
            Interval::new(a[2], b[2])
        } else {
            Interval::new(b[2], a[2])
        };
        Self { x, y, z }
    }

    pub fn from_box(box0: Self, box1: Self) -> Self {
        Self {
            x: Interval::from(box0.x, box1.x),
            y: Interval::from(box0.y, box1.y),
            z: Interval::from(box0.z, box1.z),
        }
    }

    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        let ray_origin = r.origin();
        let ray_dir = r.direction();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir[axis];

            let t0 = (ax.min() - ray_origin[axis]) * adinv;
            let t1 = (ax.max() - ray_origin[axis]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min() {
                    ray_t.set_min(t0);
                }
                if t1 < ray_t.max() {
                    ray_t.set_max(t1);
                }
            } else {
                if t1 > ray_t.min() {
                    ray_t.set_min(t1);
                }
                if t0 < ray_t.max() {
                    ray_t.set_max(t0);
                }
            }

            if ray_t.max() <= ray_t.min() {
                return false;
            }
        }
        return true;
    }

    pub fn axis_interval(&self, n: usize) -> Interval {
        match n {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("axis_interval panic"),
        }
    }

    pub fn longest_axis(&self) -> usize {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() { 0 } else { 2 }
        } else {
            if self.y.size() > self.z.size() { 1 } else { 2 }
        }
    }
}
