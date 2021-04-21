#![allow(dead_code)]
use rand::prelude::random;
use std::ops::*;
pub mod color;
pub mod point;

#[derive(Clone, Default)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    pub fn new(e: [f64; 3]) -> Self {
        Self { e }
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

    pub fn write_color(&self, samples_per_pixel: i64) {
        let mut colors = self.clone();
        let scale = 1.0 / samples_per_pixel as f64;
        colors *= scale;
        let [r, g, b] = colors.e;

        fn clamp(x: f64, min: f64, max: f64) -> f64 {
            if x < min {
                min
            } else if x > max {
                max
            } else {
                x
            }
        }

        println!(
            "{} {} {}",
            (256. * clamp(r, 0.0, 0.999)) as i32,
            (256. * clamp(g, 0.0, 0.999)) as i32,
            (256. * clamp(b, 0.0, 0.999)) as i32,
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
