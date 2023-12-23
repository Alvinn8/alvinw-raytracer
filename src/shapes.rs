use std::ops::Range;
use crate::ray::Ray;
use crate::vector::Vec3;

pub struct HitResult {
    t: f64,
    hit_point: Vec3,
    normal: Vec3,
}

impl HitResult {
    pub fn t(&self) -> f64 { self.t }
    pub fn hit_point(&self) -> Vec3 { self.hit_point }
    pub fn normal(&self) -> Vec3 { self.normal }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_range: Range<f64>) -> Option<HitResult>;
}

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self { Self { center, radius } }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_range: Range<f64>) -> Option<HitResult> {
        let oc = ray.origin() - self.center;
        let a = ray.dir().norm_sq();
        let half_b = oc.dot(ray.dir());
        let c = oc.norm_sq() - self.radius * self.radius;
        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return None
        }
        // Get smallest t in range
        let sqrt_discriminant = discriminant.sqrt();
        let mut root = (-half_b - sqrt_discriminant) / a;
        if !t_range.contains(&root) {
            root = (-half_b + sqrt_discriminant) / a;
            if !t_range.contains(&root) {
                return None;
            }
        }

        let hit_point = ray.at(root);
        Some(HitResult {
            t: root,
            hit_point,
            normal: (hit_point - self.center) / self.radius,
        })
    }
}