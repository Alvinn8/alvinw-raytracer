use std::ops::Range;
use crate::ray::Ray;
use crate::shapes::{HitResult, Hittable, InfinitePlane, Sphere, Triangle};

pub struct Scene {
    spheres: Vec<Sphere>,
    infinite_planes: Vec<InfinitePlane>,
    triangles: Vec<Triangle>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            spheres: Vec::new(),
            infinite_planes: Vec::new(),
            triangles: Vec::new(),
        }
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.spheres.push(sphere);
    }

    pub fn add_inf_plane(&mut self, plane: InfinitePlane) {
        self.infinite_planes.push(plane);
    }

    pub fn add_triangle(&mut self, triangle: Triangle) {
        self.triangles.push(triangle);
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
        for inf_plane in &self.infinite_planes {
            if let Some(hit_result) = inf_plane.hit(ray, t_range.start..closest_t) {
                closest_t = hit_result.t();
                closest = Some(hit_result);
            }
        }
        for triangle in &self.triangles {
            if let Some(hit_result) = triangle.hit(ray, t_range.start..closest_t) {
                closest_t = hit_result.t();
                closest = Some(hit_result);
            }
        }

        return closest;
    }

    pub fn count(&self) -> usize {
        self.spheres.len() + self.infinite_planes.len() + self.triangles.len()
    }
}
