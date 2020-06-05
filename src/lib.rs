pub use std::f64::consts::PI;

use rand::prelude::*;
use std::rc::Rc;

use hittable::HittableList;
use hittable::Sphere;
use material::*;
use vec3::Vec3;

pub mod camera;
pub mod hittable;
pub mod material;
pub mod ray;
pub mod vec3;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn rand_with_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min, max)
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn random_scene() -> HittableList {
    let mut scene: HittableList = Vec::new();

    let ground_material = Rc::new(Lambertian { albedo: Vec3::new(0.5, 0.5, 0.5) });
    scene.push(Rc::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    for a in -11..11 {
        for b  in -11..11 {
            let choose_mat = random_double();
            let center = Vec3::new(a as f64 + 0.9 * random_double(), 0.2, b as f64 + 0.9 * random_double());

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3::random() * Vec3::random();
                    let mat_ptr = Rc::new(Lambertian { albedo });
                    scene.push(Rc::new(Sphere::new(center, 0.2, mat_ptr)));
                } else if choose_mat < 0.95 {
                    let color = Vec3::rand_with_range(0.5, 1.0);
                    let fuzz = rand_with_range(0.0, 0.5);
                    let mat_ptr = Rc::new(Metal::new(color, fuzz));
                    scene.push(Rc::new(Sphere::new(center, 0.2, mat_ptr)));
                } else {
                    let mat_ptr = Rc::new(Dielectric::new(1.5));
                    scene.push(Rc::new(Sphere::new(center, 0.2, mat_ptr)));
                }
            }
        }
    }

    let mat_ptr = Rc::new(Dielectric::new(1.5));
    scene.push(Rc::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, mat_ptr)));
    
    let mat_ptr = Rc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    scene.push(Rc::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, mat_ptr)));
    
    let mat_ptr = Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    scene.push(Rc::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, mat_ptr)));

    scene
}