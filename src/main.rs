mod vector;
mod ray;
mod shapes;
mod scene;
mod camera;
mod util;
mod material;

use std::fs::File;
use image::ImageOutputFormat;
use crate::camera::Camera;
use crate::material::Material;
use crate::scene::Scene;
use crate::shapes::Sphere;
use crate::vector::Vec3;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 500;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    let fov = 30.0;
    let camera_from = Vec3::new(-2.0, 2.0, 1.0);
    let camera_to = Vec3::new(0.0, 0.0, -1.0);
    let camera_up = Vec3::new(0.0, 1.0, 0.0);
    let camera = Camera::new(camera_from, camera_to, camera_up, image_width, image_height, fov);

    let mut scene = Scene::new();

    let ground_material = Material::Diffuse { color: Vec3::new(0.8, 0.8, 0.0) };
    let diffuse1 = Material::Diffuse { color: Vec3::new(0.7, 0.3, 0.3) };
    let diffuse2 = Material::Diffuse { color: Vec3::new(0.3, 0.3, 0.7) };
    let metal1 = Material::Metal { color: Vec3::new(0.8, 0.8, 0.8), fuzz: 0.3 };
    let metal2 = Material::Metal { color: Vec3::new(0.8, 0.6, 0.2), fuzz: 1.0 };
    let glass1 = Material::Glass { refractive_index: 1.5 };
    let glass2 = Material::Glass { refractive_index: 1.5 };

    scene.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, ground_material));
    scene.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, glass1));
    scene.add(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.4, glass2));
    scene.add(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, metal2));

    let img = camera.render_image(&scene);

    let mut file = File::create("test.png").unwrap();
    img.write_to(&mut file, ImageOutputFormat::Png).unwrap();
}
