use std::ops::Range;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vec3;

pub struct HitResult<'a> {
    t: f64,
    hit_point: Vec3,
    normal: Vec3,
    material: &'a Material,
    front_face: bool,
}

impl<'a> HitResult<'a> {
    pub fn t(&self) -> f64 { self.t }
    pub fn hit_point(&self) -> Vec3 { self.hit_point }
    pub fn normal(&self) -> Vec3 { self.normal }
    pub fn material(&self) -> &Material { self.material }
    pub fn front_face(&self) -> bool { self.front_face }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_range: Range<f64>) -> Option<HitResult>;
}

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Self { Self { center, radius, material } }
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
        let outward_normal = (hit_point - self.center) / self.radius;
        let (normal, front_face) =  if ray.dir().dot(outward_normal) > 0.0 {
            // ray is inside the sphere
            (-outward_normal, false)
        } else {
            // ray is outside the sphere
            (outward_normal, true)
        };
        Some(HitResult {
            t: root,
            hit_point,
            normal,
            material: &self.material,
            front_face,
        })
    }
}