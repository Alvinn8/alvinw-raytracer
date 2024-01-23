#![feature(stdsimd)]

use std::arch::asm;
use std::arch::x86_64::{__m128, __m256d, _mm256_cmp_pd, _mm256_set_pd, _mm256_store_pd, _mm_add_ps, _mm_cmpeq_ps, _mm_dp_ps, _mm_extract_epi64, _mm_extract_ps, _mm_mul_ps, _mm_set_ps, _mm_sub_ps};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, Neg, Range, Sub};
use rand::random;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    data: __m128
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        unsafe {
            Self {
                data: _mm_set_ps(x as f32, y as f32, z as f32, 0.0)
            }
        }
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

    pub fn x(&self) -> f64 {
        let result: [f32; 4] = unsafe { std::mem::transmute(self.data) };
        result[3] as f64
    }
    pub fn y(&self) -> f64 {
        let result: [f32; 4] = unsafe { std::mem::transmute(self.data) };
        result[2] as f64
    }
    pub fn z(&self) -> f64 {
        let result: [f32; 4] = unsafe { std::mem::transmute(self.data) };
        result[1] as f64
    }

    pub fn is_near_zero(&self) -> bool {
        let epsilon = 1e-8;
        return self.x().abs() < epsilon && self.y().abs() < epsilon && self.z().abs() < epsilon;
    }

    pub fn norm_sq(&self) -> f64 {
        self.dot(*self)
    }
    pub fn norm(&self) -> f64 {
        self.norm_sq().sqrt()
    }
    pub fn dot(self, other: Self) -> f64 {
        let res = unsafe { _mm_dp_ps::<0xFF>(self.data, other.data) };
        let unpacked: [f32; 4] = unsafe { std::mem::transmute(res) };
        // println!("unpacked = {:?}", unpacked);
        unpacked[0] as f64
    }
    pub fn cross(self, other: Self) -> Self {
        Self::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x()
        )
    }
    pub fn normalize(self) -> Self {
        (1.0 / self.norm()) * self
    }
    pub fn reflect(&self, normal: Vec3) -> Vec3 {
        return *self - 2.0 * self.dot(normal) * normal;
    }
    pub fn refract(&self, normal: Vec3, refractive_index: f64) -> Vec3 {
        // self and normal have to be unit vectors
        let cos = (-*self).dot(normal).min(1.0);
        // Calculate vector components with some magic math (Fysik 2 bytningsindex typ)
        let perp = refractive_index * (*self + cos * normal);
        let parallel = -(1.0 - perp.norm_sq()).abs().sqrt() * normal;
        // And add
        return perp + parallel;
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x() == other.x() && self.y() == other.y() && self.z() == other.z()
    }
}

// Vector addition
impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let res = unsafe { _mm_add_ps(self.data, rhs.data) };
        Vec3 { data: res }
    }
}

// impl AddAssign for Vec3 {
//     fn add_assign(&mut self, rhs: Self) {
//         self.x() += rhs.x();
//         self.y() += rhs.y();
//         self.z() += rhs.z();
//     }
// }

// Vector subtraction
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let res = unsafe { _mm_sub_ps(self.data, rhs.data) };
        Vec3 { data: res }
    }
}

// Multiplication by scalar
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self, self, self) * rhs
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-1.0, -1.0, -1.0) * self
    }
}

// Division by scalar
impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        let scale = 1.0 / rhs;
        Vec3::new(scale, scale, scale) * self
    }
}

// impl DivAssign<f64> for Vec3 {
//     fn div_assign(&mut self, rhs: f64) {
//         self.x() /= rhs;
//         self.y() /= rhs;
//         self.z() /= rhs;
//     }
// }

// Multiplication
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        let res = unsafe { _mm_mul_ps(self.data, rhs.data) };
        Vec3 { data: res }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_components() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(vec.x(), 1.0);
        assert_eq!(vec.y(), 2.0);
        assert_eq!(vec.z(), 3.0);
    }

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
        assert!((Vec3::new(4.0, 8.0, 9.0).normalize().norm() - 1.0).abs() < 0.0001);
    }

    #[test]
    fn multiply_vector_components() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0) * Vec3::new(4.0, 5.0, 6.0), Vec3::new(4.0, 10.0, 18.0));
    }
}