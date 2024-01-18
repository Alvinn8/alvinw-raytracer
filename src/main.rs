mod vector;
mod ray;
mod shapes;
mod scene;
mod camera;
mod util;
mod material;
mod obj;

use std::fs::File;
use image::ImageOutputFormat;
use crate::camera::Camera;
use crate::material::Material;
use crate::obj::obj_to_triangles;
use crate::scene::Scene;
use crate::shapes::{InfinitePlane, Sphere, Triangle};
use crate::vector::Vec3;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 500;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    let fov = 55.0;
    let camera_from = Vec3::new(-0.8, 0.9, 1.6);
    let camera_to = Vec3::new(-0.4, 0.65, 0.0);
    let camera_up = Vec3::new(0.0, 1.0, 0.0);
    let camera = Camera::new(camera_from, camera_to, camera_up, image_width, image_height, fov);

    let mut scene = Scene::new();

    let ground_material = Material::Diffuse { color: Vec3::new(0.4, 0.7, 0.2) };
    let diffuse1 = Material::Diffuse { color: Vec3::new(0.7, 0.3, 0.3) };
    let diffuse2 = Material::Diffuse { color: Vec3::new(0.3, 0.3, 0.7) };
    let metal1 = Material::Metal { color: Vec3::new(0.8, 0.8, 0.8), fuzz: 0.3 };
    let metal2 = Material::Metal { color: Vec3::new(0.8, 0.6, 0.2), fuzz: 1.0 };
    let glass1 = Material::Glass { refractive_index: 1.5 };
    let glass2 = Material::Glass { refractive_index: 1.5 };
    let light1 = Material::Light { color: Vec3::new(1.0, 0.5, 0.5), intensity: 5.0 };

    scene.add_inf_plane(InfinitePlane::new(0.5, Vec3::new(0.0, -1.0, 0.0), ground_material));
    // scene.add_sphere(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, ground_material));
    scene.add_sphere(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, metal1));
    scene.add_sphere(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.4, glass2));
    scene.add_sphere(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, metal2));
    scene.add_sphere(Sphere::new(Vec3::new(1.0, 3.5, 2.0), 1.0, light1));
    // scene.add_triangle(Triangle::new(
    //     Vec3::new(0.0, 0.0, 0.0),
    //     Vec3::new(1.0, 0.0, 0.0),
    //     Vec3::new(0.0, 1.0, 0.0),
    //     diffuse2.clone()
    // ));

    let suzanne = obj_to_triangles("/Users/Alvin/Downloads/suzanne.obj", Vec3::new(0.0, 1.0, -2.0), diffuse2).expect("Failed to read suzanne.obj");
    suzanne.into_iter().for_each(|triangle| scene.add_triangle(triangle));
    println!("{} shapes", scene.count());

    let img = camera.render_image(&scene);

    let mut file = File::create("test.png").unwrap();
    img.write_to(&mut file, ImageOutputFormat::Png).unwrap();
}
