use std::rc::Rc;

use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub mod aabb;
pub mod bvh_node;
pub mod movable_sphere;
pub mod sphere;

use aabb::Aabb;
pub use movable_sphere::MovableSphere;
pub use sphere::Sphere;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub mat_ptr: Rc<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        p: Vec3,
        t: f64,
        u: f64,
        v: f64,
        outward_normal: Vec3,
        mat_ptr: &Rc<dyn Material>,
        ray: &Ray,
    ) -> Self {
        let front_face = Vec3::dot(ray.direction(), outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        HitRecord {
            p,
            t,
            u,
            v,
            normal,
            front_face,
            mat_ptr: Rc::clone(mat_ptr),
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, min: f64, max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb>;
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

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        if self.is_empty() {
            return None;
        }

        let mut output_box = self[0].bounding_box(t0, t1)?;

        for object in self.iter().skip(1) {
            let temp_box = object.bounding_box(t0, t1)?;
            output_box = Aabb::surrounding_box(output_box, temp_box);
        }

        Some(output_box)
    }
}
