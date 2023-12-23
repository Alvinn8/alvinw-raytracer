use std::ops::Range;
use crate::ray::Ray;
use crate::shapes::{HitResult, Hittable, Sphere};

pub struct Scene {
    spheres: Vec<Sphere>,
}

impl Scene {
    pub fn new() -> Self { Self { spheres: Vec::new() } }

    pub fn add(&mut self, sphere: Sphere) {
        self.spheres.push(sphere);
    }

    pub fn hit(&self, ray: Ray, t_range: Range<f64>) -> Option<HitResult> {
        let mut closest: Option<HitResult> = None;
        let mut closest_t = t_range.end;

        for sphere in &self.spheres {
            if let Some(hit_result) = sphere.hit(ray, t_range.start..closest_t) {
                closest_t = hit_result.t();
                closest = Some(hit_result);
            }
        }

        return closest;
    }
}
