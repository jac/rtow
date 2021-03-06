use crate::util;

use std::{f64::consts::PI, ops};

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point3 = Vec3;
pub type Colour = Vec3;

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn random() -> Self {
        Vec3 {
            x: util::random(),
            y: util::random(),
            z: util::random(),
        }
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Vec3 {
            x: util::random_range(min, max),
            y: util::random_range(min, max),
            z: util::random_range(min, max),
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() <= 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        let a = util::random_range(0.0, 2.0 * PI);
        let z = util::random_range(-1.0, 1.0);
        let r = (1.0 - z * z).sqrt();
        Vec3 {
            x: r * a.cos(),
            y: r * a.sin(),
            z,
        }
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Vec3::new(
                util::random_range(-1.0, 1.0),
                util::random_range(-1.0, 1.0),
                0.0,
            );
            if p.length_squared() <= 1.0 {
                return p;
            }
        }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Self {
        *v - 2.0 * v.dot(n) * *n
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Self {
        let cos_theta = -uv.dot(n);
        let r_out_parallel = etai_over_etat * (*uv + cos_theta * *n);
        let r_out_perp = -(1.0 - r_out_parallel.length_squared()).sqrt() * *n;
        r_out_parallel + r_out_perp
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl ops::Add<Self> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self + &rhs
    }
}

impl ops::Add<&Self> for Vec3 {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub<Self> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self - &rhs
    }
}

impl ops::Sub<&Self> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Mul<Self> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self * &rhs
    }
}

impl ops::Mul<&Self> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: &Self) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}
