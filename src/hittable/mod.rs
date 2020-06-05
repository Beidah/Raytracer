use std::rc::Rc;

use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub mod sphere;

pub use sphere::Sphere;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub mat_ptr: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, min: f64, max: f64) -> Option<HitRecord>;
}

pub type HittableList = Vec<Rc<dyn Hittable>>;

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, min: f64, max: f64) -> Option<HitRecord> {
        let mut hit_anything = None;
        let mut closest_so_far = max;

        for object in self {
            if let Some(record) = object.hit(ray, min, max) {
                if record.t < closest_so_far {
                    closest_so_far = record.t;
                    hit_anything = Some(record);
                }
            }
        }

        hit_anything
    }
}
