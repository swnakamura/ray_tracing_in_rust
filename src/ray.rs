#![allow(dead_code)]
use crate::vec3::point::Point3;
use crate::vec3::Vec3;

#[derive(Clone)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self { orig, dir }
    }
    pub fn origin(&self) -> Point3 {
        self.orig.clone()
    }
    pub fn direction(&self) -> Vec3 {
        self.dir.clone()
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.orig.clone() + self.dir.clone() * t
    }
}
