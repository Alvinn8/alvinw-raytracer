mod vector;
mod ray;
mod shapes;
mod scene;

use std::fs::File;
use std::time::Instant;
use image::{ImageOutputFormat, Rgb, RgbImage};
use crate::ray::Ray;
use crate::scene::Scene;
use crate::shapes::Sphere;
use crate::vector::Vec3;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 500;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Vec3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / (image_width as f64);
    let pixel_delta_v = viewport_v / (image_height as f64);

    let viewport_top_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - 0.5 * viewport_u - 0.5 * viewport_v;
    let top_left_pixel_pos = viewport_top_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut scene = Scene::new();
    scene.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    scene.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    let start = Instant::now();

    let mut img = RgbImage::new(image_width, image_height);
    for y in 0..image_height {
        print!("\ry = {}", y);
        for x in 0..image_width {
            let viewport_pixel = top_left_pixel_pos + (x as f64 * pixel_delta_u) + (y as f64 * pixel_delta_v);
            let ray_dir = viewport_pixel - camera_center;
            let ray = Ray::new(camera_center, ray_dir);
            let color = ray_color(ray, &scene);

            img.put_pixel(x, y, color);
        }
    }

    let elapsed = start.elapsed();
    println!("\rDone in {:.2?}", elapsed);

    let mut file = File::create("test.png").unwrap();
    img.write_to(&mut file, ImageOutputFormat::Png).unwrap();
}

fn color(r: f64, g: f64, b: f64) -> Rgb<u8> {
    return Rgb([
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
    ])
}

fn ray_color(ray: Ray, scene: &Scene) -> Rgb<u8> {
    let hit_result = scene.hit(ray, 0.0..f64::INFINITY);
    if let Some(hit_result) = hit_result {
        let normal = hit_result.normal();
        return color(
            0.5 * normal.x() + 0.5,
            0.5 * normal.y() + 0.5,
            0.5 * normal.z() + 0.5,
        );
    }

    let dir_n = ray.dir().normalize();
    let a = 0.5 * (dir_n.y() + 1.0);
    color(
        (1.0-a) * 1.0 + 0.7 * a,
        (1.0-a) * 1.0 + 1.0 * a,
        (1.0-a) * 1.0 + 1.0 * a,
    )
}
