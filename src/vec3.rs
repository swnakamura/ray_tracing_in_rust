#![allow(dead_code)]
use std::ops::*;
pub mod color;
pub mod point;

#[derive(Clone)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    pub fn new(e: [f64; 3]) -> Self {
        Self { e }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.e[0] * rhs.e[0] + self.e[1] * rhs.e[1] + self.e[2] * rhs.e[2]
    }

    pub fn cross(&self, rhs: &Self) -> Vec3 {
        Vec3 {
            e: [
                self.e[1] * rhs.e[2] - self.e[2] * rhs.e[1],
                self.e[2] * rhs.e[0] - self.e[0] * rhs.e[2],
                self.e[0] * rhs.e[1] - self.e[1] * rhs.e[0],
            ],
        }
    }

    pub fn normalize(self) -> Self {
        let l = self.length();
        self / l
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn write_color(&self) {
        println!(
            "{} {} {}",
            (255.999 * self.x()) as i32,
            (255.999 * self.y()) as i32,
            (255.999 * self.z()) as i32,
        );
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(mut self, rhs: f64) -> Self::Output {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
        self
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(mut self, rhs: f64) -> Self::Output {
        self.e[0] /= rhs;
        self.e[1] /= rhs;
        self.e[2] /= rhs;
        self
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(mut self, rhs: Vec3) -> Self::Output {
        self.e[0] *= rhs.e[0];
        self.e[1] *= rhs.e[1];
        self.e[2] *= rhs.e[2];
        self
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Self;
    fn div(mut self, rhs: Vec3) -> Self::Output {
        self.e[0] /= rhs.e[0];
        self.e[1] /= rhs.e[1];
        self.e[2] /= rhs.e[2];
        self
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(mut self, rhs: Vec3) -> Self::Output {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
        self
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(mut self, rhs: Vec3) -> Self::Output {
        self.e[0] -= rhs.e[0];
        self.e[1] -= rhs.e[1];
        self.e[2] -= rhs.e[2];
        self
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.e[0] = -self.e[0];
        self.e[1] = -self.e[1];
        self.e[2] = -self.e[2];
        self
    }
}
