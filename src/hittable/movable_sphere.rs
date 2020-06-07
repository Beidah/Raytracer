use std::rc::Rc;

use super::{aabb::Aabb, sphere::get_sphere_uv, HitRecord, Hittable};
use crate::vec3::Vec3;
use crate::{material::Material, ray::Ray};

pub struct MovableSphere {
    center0: Vec3,
    center1: Vec3,
    time0: f64,
    time1: f64,
    radius: f64,
    mat_ptr: Rc<dyn Material>,
}

impl MovableSphere {
    pub fn new(
        center0: Vec3,
        center1: Vec3,
        time0: f64,
        time1: f64,
        radius: f64,
        mat_ptr: Rc<dyn Material>,
    ) -> Self {
        MovableSphere {
            center0,
            center1,
            time0,
            time1,
            radius,
            mat_ptr: Rc::clone(&mat_ptr),
        }
    }

    pub fn center(&self, time: f64) -> Vec3 {
        self.center0
            + (time - self.time0) / (self.time1 - self.time0) * (self.center1 - self.center0)
    }
}

impl Hittable for MovableSphere {
    fn hit(&self, ray: Ray, min: f64, max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center(ray.time());
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
                let normal = (p - self.center(ray.time())) / self.radius;
                let (u, v) = get_sphere_uv(&normal);
                let record = HitRecord::new(p, t, u, v, normal, &self.mat_ptr, &ray);

                return Some(record);
            }

            let temp = (-half_b + root) / a;
            if temp < max && temp > min {
                let t = temp;
                let p = ray.at(t);
                let normal = (p - self.center(ray.time())) / self.radius;
                let (u, v) = get_sphere_uv(&normal);
                let record = HitRecord::new(p, t, u, v, normal, &self.mat_ptr, &ray);

                return Some(record);
            }
        }

        None
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        let box0 = Aabb::new(
            self.center(t0) - Vec3(self.radius, self.radius, self.radius),
            self.center(t0) + Vec3(self.radius, self.radius, self.radius),
        );

        let box1 = Aabb::new(
            self.center(t1) - Vec3(self.radius, self.radius, self.radius),
            self.center(t1) + Vec3(self.radius, self.radius, self.radius),
        );

        let output_box = Aabb::surrounding_box(box0, box1);

        Some(output_box)
    }
}
