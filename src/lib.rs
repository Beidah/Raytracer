pub use std::f64::consts::PI;

use rand::prelude::*;
use std::rc::Rc;

use hittable::{bvh_node::BvhNode, HittableList, Sphere, rectangle::XYRect};
use material::*;
use texture::NoiseTexture;
use vec3::Vec3;

pub mod camera;
pub mod hittable;
pub mod material;
pub mod noise;
pub mod ray;
pub mod texture;
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

    let ground_material = Rc::new(Lambertian::from((Vec3(0.2, 0.3, 0.1), Vec3(0.9, 0.9, 0.9))));

    scene.push(Rc::new(Sphere::new(
        Vec3(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Vec3(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.75 {
                    let albedo = Vec3::random() * Vec3::random();
                    let mat_ptr = Rc::new(Lambertian::from(albedo));
                    scene.push(Rc::new(Sphere::new(center, 0.2, mat_ptr)));
                // } else  if choose_mat < 0.8 {
                //     let albedo = Vec3::random() * Vec3::random();
                //     let mat_ptr = Rc::new(Lambertian { albedo });
                //     let center2 = center + Vec3::new(0.0, rand_with_range(0.0, 0.5), 0.0);
                //     scene.push(Rc::new(MovableSphere::new(center, center2, 0.0, 1.0, 0.2, mat_ptr)));
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
    scene.push(Rc::new(Sphere::new(Vec3(0.0, 1.0, 0.0), 1.0, mat_ptr)));

    let mat_ptr = Rc::new(Lambertian::from((0.4, 0.2, 0.1)));
    scene.push(Rc::new(Sphere::new(Vec3(-4.0, 1.0, 0.0), 1.0, mat_ptr)));

    let mat_ptr = Rc::new(Metal::new(Vec3(0.7, 0.6, 0.5), 0.0));
    scene.push(Rc::new(Sphere::new(Vec3(4.0, 1.0, 0.0), 1.0, mat_ptr)));

    let scene = Rc::new(BvhNode::new(scene, 0.0, 1.0));

    vec![scene]
}

pub fn two_perlin_spheres() -> HittableList {
    let mut objects = HittableList::new();

    let pertext = Rc::new(NoiseTexture::new(4.0));

    objects.push(Rc::new(Sphere::new(
        Vec3(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new(pertext.clone())),
    )));

    objects.push(Rc::new(Sphere::new(
        Vec3(0.0, 2.0, 0.0),
        2.0,
        Rc::new(Lambertian::new(pertext)),
    )));

    objects
}

pub fn earth() -> HittableList {
    let earth_texture = Rc::new(Lambertian::from(
        "F:\\workspace\\rust\\raytracer\\res\\earthmap.jpg",
    ));
    let globe = Rc::new(Sphere::new(Vec3(0.0, 0.0, 0.0), 2.0, earth_texture));

    vec![globe]
}

pub fn simple_light() -> HittableList {
    let mut objects = HittableList::new();

    let pertext = Rc::new(NoiseTexture::new(4.0));

    objects.push(Rc::new(Sphere::new(
        Vec3(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new(pertext.clone())),
    )));

    objects.push(Rc::new(Sphere::new(
        Vec3(0.0, 2.0, 0.0),
        2.0,
        Rc::new(Lambertian::new(pertext)),
    )));

    let diffuse_light = Rc::new(DiffuseLight::from((4.0, 4.0, 4.0)));

    objects.push(Rc::new(Sphere::new(
        Vec3(0.0, 7.0, 0.0),
        2.0,
        diffuse_light.clone(),
    )));

    objects.push(Rc::new(XYRect::new(
        3.0, 5.0,
        1.0, 3.0, -2.0,
        diffuse_light
    )));

    objects
}
