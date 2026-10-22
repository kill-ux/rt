use std::ops::{Add, Sub};

use rand::random;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3::new(0., 0., 0.);

    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn length(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        let len = self.length();
        Vec3::new(self.x / len, self.y / len, self.z / len)
    }

    pub fn random_unit() -> Vec3 {
        loop {
            let p = Vec3::new(
                random::<f64>() * 2. - 1.,
                random::<f64>() * 2. - 1.,
                random::<f64>() * 2. - 1.,
            );

            let len_sq = p.dot(&p);

            if len_sq > 1e-12 && len_sq <= 1. {
                return p.normalize();
            }
        }
    }

    pub fn mul_elem(&self, other: Vec3) -> Vec3 {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }

    /// R=D−2(D⋅N)N
    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        *self - *normal * (2.0 * self.dot(normal))
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Self) -> Self::Output {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Self) -> Self::Output {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, scalar: f64) -> Vec3 {
        Vec3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}
