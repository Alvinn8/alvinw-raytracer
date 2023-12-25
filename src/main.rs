mod vector;
mod ray;
mod shapes;
mod scene;
mod camera;
mod util;

use std::fs::File;
use image::ImageOutputFormat;
use crate::camera::Camera;
use crate::scene::Scene;
use crate::shapes::Sphere;
use crate::vector::Vec3;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 500;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    let camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), image_width, image_height);

    let mut scene = Scene::new();
    scene.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    scene.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    let img = camera.render_image(&scene);

    let mut file = File::create("test.png").unwrap();
    img.write_to(&mut file, ImageOutputFormat::Png).unwrap();
}
