use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, PartialEq, Debug, PartialOrd)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn normalize(self) -> Self {
        self / self.abs()
    }

    fn abs(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y + rhs.y + self.z * rhs.z
    }
}
impl From<&[f32]> for Vec3 {
    fn from(value: &[f32]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}
impl Add for Vec3 {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self
    }
}
impl Sub for Vec3 {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self
    }
}
impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(mut self, rhs: f32) -> Self::Output {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self
    }
}
impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        self * (1f32 / rhs)
    }
}
