use super::{aabb::*, HitRecord, Hittable, HittableList};
use crate::ray::Ray;
use rand::Rng;
use std::rc::Rc;

pub struct BvhNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bounding_box: Aabb,
}

impl BvhNode {
    pub fn new(mut list: HittableList, time0: f64, time1: f64) -> Self {
        Self::create(&mut list, time0, time1)
    }

    fn create(objects: &mut [Rc<dyn Hittable>], time0: f64, time1: f64) -> Self {
        let mut rng = rand::thread_rng();
        let axis = rng.gen_range(0, 2);

        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            _ => box_z_compare,
        };

        let (left, right) = match objects.len() {
            1 => (Rc::clone(&objects[0]), Rc::clone(&objects[0])),
            2 => match comparator(&objects[0], &objects[1]) {
                std::cmp::Ordering::Less => (Rc::clone(&objects[0]), Rc::clone(&objects[1])),
                _ => (Rc::clone(&objects[1]), Rc::clone(&objects[0])),
            },
            _ => {
                objects.sort_by(comparator);
                let mid = objects.len() / 2;
                let left: Rc<dyn Hittable> =
                    Rc::new(Self::create(&mut objects[0..mid], time0, time1));
                let right: Rc<dyn Hittable> =
                    Rc::new(Self::create(&mut objects[mid..], time0, time1));
                (left, right)
            }
        };

        let box_left = left.bounding_box(time0, time1).unwrap();
        let box_right = right.bounding_box(time0, time1).unwrap();

        let bounding_box = Aabb::surrounding_box(box_left, box_right);

        Self {
            left,
            right,
            bounding_box,
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: Ray, min: f64, max: f64) -> Option<HitRecord> {
        if !self.bounding_box.hit(&ray, min, max) {
            return None;
        }

        match self.left.hit(ray, min, max) {
            Some(record) => {
                let max = record.t;
                match self.right.hit(ray, min, max) {
                    Some(record) => Some(record),
                    None => Some(record),
                }
            }
            None => self.right.hit(ray, min, max),
        }
    }
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        Some(self.bounding_box)
    }
}
