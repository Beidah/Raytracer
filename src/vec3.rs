use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::*;
use image::{Rgb, Rgba};

pub type Color = Vec3;

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3(x, y, z)
    }

    pub fn random() -> Self {
        Vec3(random_double(), random_double(), random_double())
    }

    pub fn rand_with_range(min: f64, max: f64) -> Self {
        Vec3(
            rand_with_range(min, max),
            rand_with_range(min, max),
            rand_with_range(min, max),
        )
    }

    pub fn rand_unit_vector() -> Self {
        let a = rand_with_range(0.0, 2.0 * PI);
        let z = rand_with_range(-1.0, 1.0);
        let r = (1.0 - z * z).sqrt();
        Vec3(r * a.cos(), r * a.sin(), z)
    }

    pub fn rand_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::rand_with_range(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn rand_in_unit_disk() -> Self {
        loop {
            let p = Vec3(rand_with_range(-1.0, 1.0), rand_with_range(-1.0, 1.0), 0.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn get(&self, index: usize) -> f64 {
        match index {
            0 => self.0,
            1 => self.1,
            2 => self.2,
            _ => panic!("Index out of bounds."),
        }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn dot(lhs: Self, rhs: Self) -> f64 {
        lhs.0 * rhs.0 + lhs.1 * rhs.1 + lhs.2 * rhs.2
    }

    pub fn cross(lhs: Vec3, rhs: Vec3) -> Self {
        Vec3(
            lhs.1 * rhs.2 - lhs.2 * rhs.1,
            lhs.2 * rhs.0 - lhs.0 * rhs.2,
            lhs.0 * rhs.1 - lhs.1 * rhs.0,
        )
    }

    pub fn reflect(v: Vec3, normal: Vec3) -> Vec3 {
        v - 2.0 * Self::dot(v, normal) * normal
    }

    pub fn unit_vector(v: Vec3) -> Self {
        v / v.length()
    }
}

impl From<(u8, u8, u8)> for Vec3 {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        let color_scale = 1.0 / 255.0;

        let r = r as f64 * color_scale;
        let g = g as f64 * color_scale;
        let b = b as f64 * color_scale;

        Vec3(r, g, b)
    }
}

impl From<Rgba<u8>> for Vec3 {
    fn from(pixel: Rgba<u8>) -> Self {
        Vec3::from((pixel[0], pixel[1], pixel[2]))
    }
}

impl From<Rgb<u8>> for Vec3 {
    fn from(pixel: Rgb<u8>) -> Self {
        Vec3::from((pixel[0], pixel[1], pixel[2]))
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Add<&Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Vec3(self.0 + rhs, self.1 + rhs, self.2 + rhs)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2);
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2);
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<&f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &f64) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Vec3 {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self(self.0 * rhs, self.1 * rhs, self.2 * rhs);
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Self(self.0 / rhs, self.1 / rhs, self.2 / rhs);
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = Vec3::dot(-uv, n);
    let r_out_parellel = etai_over_etat * (uv + cos_theta * n);
    let r_out_perp = -f64::sqrt(1.0 - r_out_parellel.length_squared()) * n;
    r_out_parellel + r_out_perp
}
