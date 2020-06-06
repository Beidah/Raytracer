use std::rc::Rc;

use super::{aabb::Aabb, HitRecord, Hittable};
use crate::vec3::Vec3;
use crate::{material::Material, ray::Ray};

#[derive(Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
    mat_ptr: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat_ptr: Rc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            mat_ptr: Rc::clone(&mat_ptr),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, min: f64, max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = Vec3::dot(ray.direction(), ray.direction());
        let half_b = Vec3::dot(oc, ray.direction());
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;

            if temp < max && temp > min {
                let t = temp;
                let p = ray.at(t);
                let normal = (p - self.center) / self.radius;
                let record = HitRecord::new(p, t, normal, &self.mat_ptr, &ray);

                return Some(record);
            }

            let temp = (-half_b + root) / a;
            if temp < max && temp > min {
                let t = temp;
                let p = ray.at(t);
                let normal = (p - self.center) / self.radius;
                let record = HitRecord::new(p, t, normal, &self.mat_ptr, &ray);

                return Some(record);
            }
        }

        None
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        let output_box = Aabb::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        );
        Some(output_box)
    }
}
