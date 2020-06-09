use super::*;

pub struct XYRect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    mat_ptr: Rc<dyn Material>,
}

impl XYRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mat_ptr: Rc<dyn Material>) -> Self { Self { x0, x1, y0, y1, k, mat_ptr } }
}

impl Hittable for XYRect {
    fn hit(&self, ray: Ray, min: f64, max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin().z()) / ray.direction().z();

        if t < min || t > max {
            return None;
        }

        let x = ray.origin().x() + t * ray.direction().x();
        let y = ray.origin().y() + t * ray.direction().y();

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);

        let outward_normal = Vec3(0.0, 0.0, 1.0);

        let p = ray.at(t);

        let record = HitRecord::new(p, t, u, v, outward_normal, &self.mat_ptr, &ray);

        Some(record)
    }
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        // The bounding box must have non-zero width in each dimension, so pad the Z
        // dimension a small amount.
        Some(aabb::Aabb::new(Vec3(self.x0, self.y0, self.k-0.0001), Vec3(self.x1, self.y1, self.k+0.0001)))
    }
    
}