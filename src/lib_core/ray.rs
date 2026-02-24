use crate::lib_core::point::Point3;
use crate::lib_core::vec::Vec3;

pub struct Ray {
    org: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            org: origin,
            dir: direction,
        }
    }

    pub fn default() -> Self {
        Self {
            org: Vec3::default(),
            dir: Vec3::default(),
        }
    }

    pub fn origin(&self) -> Point3 {
        self.org
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.org + self.dir * t
    }
}
