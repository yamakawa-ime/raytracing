use crate::lib_core::point::Point3;
use crate::lib_core::vec::Vec3;

pub struct Ray {
    org: Point3,
    dir: Vec3,
    tm: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: f64) -> Self {
        Self {
            org: origin,
            dir: direction,
            tm: time,
        }
    }

    pub fn default() -> Self {
        Self {
            org: Vec3::zero(),
            dir: Vec3::zero(),
            tm: 0.0,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.org
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn time(&self) -> f64 {
        self.tm
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.org + self.dir * t
    }
}
