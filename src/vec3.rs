#![allow(dead_code)]
use rand::prelude::random;
use std::iter::Sum;
use std::ops::*;
pub mod color;
pub mod point;

#[derive(Clone, Copy, Default, std::fmt::Debug)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Self { e: [a, b, c] }
    }
    pub fn random() -> Self {
        Self {
            e: [random(), random(), random()],
        }
    }
    pub fn random_in_range(min: f64, max: f64) -> Self {
        fn linearmap(val: f64, min: f64, max: f64) -> f64 {
            val * (max - min) + min
        }
        Self {
            e: [
                linearmap(random(), min, max),
                linearmap(random(), min, max),
                linearmap(random(), min, max),
            ],
        }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn length_squared(&self) -> f64 {
        self.e.iter().map(|x| x * x).sum()
    }
    pub fn near_zero(&self) -> bool {
        let eps = 1e-8;
        return self.e.iter().all(|&x| x.abs() < eps);
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

    pub fn reflect(&self, n: &Vec3) -> Vec3 {
        *self - *n * (self.dot(n) * 2.)
    }
    pub fn refract(&self, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let normalized_self = self.normalize();
        let cos_theta = -normalized_self.dot(&n);
        let r_out_perp = (normalized_self + *n * cos_theta) * etai_over_etat;
        let r_out_parallel = -*n * (1. - r_out_perp.length_squared()).sqrt();
        return r_out_perp + r_out_parallel;
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

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self {
            e: [self.e[0] * other, self.e[1] * other, self.e[2] * other],
        }
    }
}

impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vec3::new(0., 0., 0.), Add::add)
    }
}
