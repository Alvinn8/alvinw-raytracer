use rand::random;
use crate::ray::Ray;
use crate::shapes::HitResult;
use crate::vector::Vec3;

pub enum Material {
    Diffuse {
        color: Vec3,
    },
    Metal {
        color: Vec3,
        fuzz: f64,
    },
    Glass {
        refractive_index: f64,
    }
}

impl Material {
    pub fn scatter(&self, ray: Ray, hit_result: &HitResult) -> Option<Scatter> {
        return Some(match self {
            Material::Diffuse { color } => {
                let normal = hit_result.normal();
                let mut bounce_dir = normal + Vec3::random().normalize();

                // Avoid division by zero and other problems
                if bounce_dir.is_near_zero() {
                    bounce_dir = normal;
                }

                let bounce_ray = Ray::new(hit_result.hit_point(), bounce_dir);
                Scatter {
                    ray: bounce_ray,
                    attenuation: *color
                }
            }
            Material::Metal { color, fuzz } => {
                let reflected = ray.dir().reflect(hit_result.normal());
                let mut fuzz_vector = Vec3::random();
                fuzz_vector = (fuzz / fuzz_vector.norm()) * fuzz_vector;
                let dir = reflected + fuzz_vector;
                if dir.dot(hit_result.normal()) < 0.0 {
                    return None;
                }
                let ray = Ray::new(hit_result.hit_point(), dir);
                Scatter {
                    ray,
                    attenuation: *color
                }
            }
            Material::Glass { refractive_index } => {
                let refraction_ratio = if hit_result.front_face() {
                    1.0 / refractive_index
                } else {
                    *refractive_index
                };

                let dir = ray.dir().normalize();
                let normal = hit_result.normal();
                let cos = (-dir).dot(normal).min(1.0);
                let sin = (1.0 - cos * cos).sqrt();

                let rand: f64 = random();
                let bounce_dir = if refraction_ratio * sin > 1.0 || reflectance(cos, refraction_ratio) > rand {
                    // Total internal reflection
                    dir.reflect(normal)
                } else {
                    dir.refract(normal, refraction_ratio)
                };

                let ray = Ray::new(hit_result.hit_point(), bounce_dir);

                Scatter {
                    ray,
                    attenuation: Vec3::new(1.0, 1.0, 1.0)
                }
            }
        })
    }
}

fn reflectance(cos: f64, refractive_index: f64) -> f64 {
    // More magic maths. Schlick's approximation for reflectance.
    let mut r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cos).powi(5);
}

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Vec3,
}