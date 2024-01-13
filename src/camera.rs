use std::time::Instant;
use image::{ImageBuffer, Rgb, RgbImage};
use rand::random;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::vector::Vec3;

pub struct Camera {
    image_width: u32,
    image_height: u32,
    center: Vec3,
    viewport_width: f64,
    viewport_height: f64,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    top_left_pixel_pos: Vec3,
    max_depth: u32,
}

impl Camera {
    pub fn new(camera_center: Vec3, image_width: u32, image_height: u32) -> Self {
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);

        let viewport_top_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - 0.5 * viewport_u - 0.5 * viewport_v;
        let top_left_pixel_pos = viewport_top_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            image_width,
            image_height,
            center: camera_center,
            viewport_width,
            viewport_height,
            pixel_delta_u,
            pixel_delta_v,
            top_left_pixel_pos,
            max_depth: 50,
        }
    }

    pub fn render_image(&self, scene: &Scene) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let start = Instant::now();

        let mut img = RgbImage::new(self.image_width, self.image_height);
        for y in 0..self.image_height {
            print!("\r{:.2}%   ", (y as f64 / self.image_height as f64) * 100.0);
            for x in 0..self.image_width {
                // Average colors (anti-aliasing)
                let mut color = Vec3::zero();
                for i in 0..10 {
                    let ray = self.ray_rand(x, y);
                    let color_i = self.ray_color(ray, &scene, self.max_depth);
                    color += color_i;
                }
                color /= 10.0;

                let rgb = to_rgb(color.x(), color.y(), color.z());
                img.put_pixel(x, y, rgb);
            }
        }

        let elapsed = start.elapsed();
        println!("\nDone in {:.2?}", elapsed);

        img
    }

    fn ray_rand(&self, x: u32, y: u32) -> Ray {
        let viewport_pixel = self.top_left_pixel_pos + (x as f64 * self.pixel_delta_u) + (y as f64 * self.pixel_delta_v);
        let delta_x = random::<f64>() - 0.5;
        let delta_y = random::<f64>() - 0.5;
        let random_pixel = viewport_pixel + (delta_x * self.pixel_delta_u) + (delta_y * self.pixel_delta_v);

        let ray_dir = random_pixel - self.center;
        Ray::new(self.center, ray_dir)
    }

    fn ray_color(&self, ray: Ray, scene: &Scene, depth: u32) -> Vec3 {
        if depth < 1 {
            return Vec3::zero();
        }
        let hit_result = scene.hit(ray, 0.001..f64::INFINITY);
        if let Some(hit_result) = hit_result {
            let normal = hit_result.normal();
            let bounce_dir = normal + Vec3::random().normalize();
            let bounce_ray = Ray::new(hit_result.hit_point(), bounce_dir);
            return 0.5 * self.ray_color(bounce_ray, scene, depth - 1);
        }

        let dir_n = ray.dir().normalize();
        let a = 0.5 * (dir_n.y() + 1.0);
        Vec3::new(
            (1.0-a) * 1.0 + 0.7 * a,
            (1.0-a) * 1.0 + 1.0 * a,
            (1.0-a) * 1.0 + 1.0 * a,
        )
    }
}

fn to_rgb(r: f64, g: f64, b: f64) -> Rgb<u8> {
    return Rgb([
        (gamma_correction(r) * 255.0) as u8,
        (gamma_correction(g) * 255.0) as u8,
        (gamma_correction(b) * 255.0) as u8,
    ])
}

fn gamma_correction(value: f64) -> f64 {
    return value.sqrt();
}
