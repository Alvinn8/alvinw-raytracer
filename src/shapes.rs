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

pub struct InfinitePlane {
    dist: f64,
    normal: Vec3,
    material: Material,
}

impl InfinitePlane {
    pub fn new(dist: f64, normal: Vec3, material: Material) -> InfinitePlane {
        InfinitePlane {
            dist,
            normal: normal.normalize(),
            material,
        }
    }
}

impl Hittable for InfinitePlane {
    fn hit(&self, ray: Ray, t_range: Range<f64>) -> Option<HitResult> {
        let denominator = ray.dir().dot(self.normal);
        if denominator == 0.0 {
            return None;
        }
        let numerator = self.dist - ray.origin().dot(self.normal);

        let t = numerator / denominator;
        if !t_range.contains(&t) {
            return None;
        }
        Some(HitResult {
            t,
            hit_point: ray.at(t),
            normal: -self.normal,
            material: &self.material,
            front_face: false,
        })
    }
}

#[derive(Debug)]
pub struct Triangle {
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
    material: Material,
}

impl Triangle {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3, material: Material) -> Self {
        Self { v0, v1, v2, material, }
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: Ray, t_range: Range<f64>) -> Option<HitResult> {
        // https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;
        let ray_cross_e2 = ray.dir().cross(edge2);
        let det = edge1.dot(ray_cross_e2);

        // Triangle is parallel to ray
        if det.abs() < f64::EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;
        let s = ray.origin() - self.v0;
        let u = inv_det * s.dot(ray_cross_e2);
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let s_cross_e1 = s.cross(edge1);
        let v = inv_det * ray.dir().dot(s_cross_e1);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        // Cramers regel wow!
        let t = edge2.dot(s_cross_e1) * inv_det;
        
        if !t_range.contains(&t) {
            return None;
        }

        Some(HitResult {
            t,
            hit_point: ray.at(t),
            normal: edge1.cross(edge2).normalize(),
            material: &self.material,
            front_face: false,
        })
    }
}