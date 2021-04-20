use super::point::Point3;
use super::Vec3;

struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
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
