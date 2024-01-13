use std::ops::{Add, AddAssign, Div, DivAssign, Mul, Neg, Range, Sub};
use rand::random;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn zero() -> Self { Self::new(0.0, 0.0, 0.0) }
    pub fn random() -> Self {
        let (rx, ry, rz): (f64, f64, f64) = random();
        Self::new(rx * 2.0 - 1.0, ry * 2.0 - 1.0, rz * 2.0 - 1.0)
    }
    pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
        let vec = Vec3::random().normalize();
        if vec.dot(normal) > 0.0 {
            vec
        } else {
            -vec
        }
    }

    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }
    pub fn z(&self) -> f64 { self.z }

    pub fn norm_sq(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn norm(&self) -> f64 {
        self.norm_sq().sqrt()
    }
    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x  - self.x * other.z,
            self.x * other.y - self.y * other.x
        )
    }
    pub fn normalize(self) -> Self {
        (1.0 / self.norm()) * self
    }
}

// Vector addition
impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
        )
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

// Vector subtraction
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
        )
    }
}

// Multiplication by scalar
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self * rhs.x,
            self * rhs.y,
            self * rhs.z,
        )
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self::new(
            -self.x,
            -self.y,
            -self.z,
        )
    }
}

// Division by scalar
impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Self::new(
            self.x / rhs,
            self.y / rhs,
            self.z / rhs,
        )
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_addition() {
        assert_eq!(Vec3::new(1.0, 1.0, 1.0) + Vec3::new(2.0, 2.0, 2.0), Vec3::new(3.0, 3.0, 3.0));
    }

    #[test]
    fn norm_one() {
        assert_eq!(Vec3::new(1.0, 0.0, 0.0).norm(), 1.0);
    }

    #[test]
    fn cross_orthogonal_dot() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(0.0, 1.0, 0.0);
        let c = a.cross(b);

        assert_eq!(c, Vec3::new(0.0, 0.0, 1.0));
        assert_eq!(a.dot(c), 0.0);
        assert_eq!(b.dot(c), 0.0);
    }

    #[test]
    fn zero_norm() {
        assert_eq!(Vec3::zero().norm(), 0.0);
    }

    #[test]
    fn scale_by_five() {
        assert_eq!(5.0 * Vec3::new(2.0, 2.0, 2.0), Vec3::new(10.0, 10.0, 10.0));
    }

    #[test]
    fn normalize() {
        assert!((Vec3::new(4.0, 8.0, 9.0).normalize().norm() - 1.0).abs() < f64::EPSILON);
    }
}