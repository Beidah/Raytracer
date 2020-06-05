use super::{HitRecord, Hittable};

use crate::vec3::Vec3;
use crate::{material::Material, ray::Ray};
use std::rc::Rc;

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
                let mut record = HitRecord {
                    t,
                    p,
                    normal,
                    mat_ptr: self.mat_ptr.clone(),
                    front_face: false
                };
                record.set_face_normal(ray, normal);

                return Some(record);
            }
            
            let temp = (-half_b + root) / a;
            if temp < max && temp > min {
                let t = temp;
                let p = ray.at(t);
                let normal = (p - self.center) / self.radius;
                let mut record = HitRecord {
                    t,
                    p,
                    normal,
                    mat_ptr: self.mat_ptr.clone(),
                    front_face: false
                };
                record.set_face_normal(ray, normal);

                return Some(record);
            }
        }
        
        None
    }
}
