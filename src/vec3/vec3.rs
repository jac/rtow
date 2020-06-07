use std::convert::From;
use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
   pub x: f64,
   pub y: f64,
   pub z: f64
}
pub type Point3 = Vec3;
pub type Colour = Vec3;

impl Vec3 {
    /// Create a Vec3{x: 0.0, y: 0.0, z: 0.0}
    pub fn new() -> Vec3 {
        Vec3{x: 0.0, y: 0.0, z: 0.0}
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(lhs: Self, rhs: Self) -> f64 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    pub fn cross(lhs: Self, rhs: Self) -> Self {
        Vec3 {
            x: lhs.y * rhs.z - lhs.z * rhs.y,
            y: lhs.z * rhs.x - lhs.x * rhs.z,
            z: lhs.x * rhs.y - lhs.y * rhs.x
        }
    }

    pub fn unit_vector(v: Self) -> Self {
        v / v.length()
    }
}

impl ToString for Vec3 {
    fn to_string(&self) -> String {
        format!("{} {} {}\n", self.x, self.y, self.z)
    }
}

impl From<(f64, f64, f64)> for Vec3 {
    fn from(src: (f64, f64, f64)) -> Vec3 {
        Vec3{x: src.0, y: src.1, z: src.2}
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;
    
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Out of bounds Vec3 index `{}`", index)
        }
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Out of bounds Vec3 index `{}`", index)
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {x: -self.x, y: -self.y, z: -self.z}
    }
}

impl ops::Add<Self> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl ops::Sub<Self> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl ops::Mul<Self> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0/rhs)
    }
}

impl ops::AddAssign<Self> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::SubAssign<Self> for Vec3 {
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
        *self *= 1.0/rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_squared() {
        assert_eq!(Vec3::from((1.,2.,3.)).length_squared(), 14.);
    }

    #[test]
    fn dot() {
        assert_eq!(Vec3::dot(Vec3::from((1.,2.,3.)), Vec3::from((1.,5.,7.))), 32.);
    }

    #[test]
    fn cross() {
        assert_eq!(Vec3::cross(Vec3::from((1.,2.,3.)), Vec3::from((1.,5.,7.))), Vec3::from((-1., -4., 3.)))
    }
}