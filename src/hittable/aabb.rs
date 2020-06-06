use crate::{ray::Ray, vec3::Vec3};

use super::Hittable;
use std::{cmp::Ordering, rc::Rc};

#[derive(Debug, Default, Copy, Clone)]
pub struct Aabb {
    min: Vec3,
    max: Vec3,
}

impl Aabb {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Aabb { min, max }
    }

    pub fn surrounding_box(box0: Self, box1: Self) -> Self {
        let min = Vec3::new(
            f64::min(box0.min().x(), box1.min().x()),
            f64::min(box0.min().y(), box1.min().y()),
            f64::min(box0.min().z(), box1.min().z()),
        );

        let max = Vec3::new(
            f64::max(box0.max().x(), box1.max().x()),
            f64::max(box0.max().y(), box1.max().y()),
            f64::max(box0.max().z(), box1.max().z()),
        );

        Aabb { min, max }
    }

    pub fn min(&self) -> Vec3 {
        self.min
    }

    pub fn max(&self) -> Vec3 {
        self.max
    }

    pub fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> bool {
        for i in 0..3 {
            let inv_d = 1.0 / ray.direction().get(i);
            let mut t0 = (self.min.get(i) - ray.origin().get(i)) * inv_d;
            let mut t1 = (self.max.get(i) - ray.origin().get(i)) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            let tmin = f64::max(t0, tmin);
            let tmax = f64::min(t1, tmax);

            if tmax <= tmin {
                return false;
            }
        }

        true
    }
}

fn box_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis: usize) -> Ordering {
    let box_a = a.bounding_box(0.0, 0.0).unwrap();
    let box_b = b.bounding_box(0.0, 0.0).unwrap();

    box_a
        .min()
        .get(axis)
        .partial_cmp(&box_b.min().get(axis))
        .unwrap()
}

pub fn box_x_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 0)
}

pub fn box_y_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 1)
}

pub fn box_z_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 2)
}
