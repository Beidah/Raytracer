use std::rc::Rc;

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{refract, Vec3};
use crate::{
    clamp,
    texture::{CheckerTexture, SolidColor, Texture},
};

fn schlick(cosine: f64, ref_ind: f64) -> f64 {
    let r0 = (1.0 - ref_ind) / (1.0 + ref_ind);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    pub albedo: Rc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Rc<dyn Texture>) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction = record.normal + Vec3::rand_unit_vector();
        *scattered = Ray::new(record.p, scatter_direction, ray_in.time());
        *attenuation = self.albedo.value(record.u, record.v, record.p);
        true
    }
}

impl From<(f64, f64, f64)> for Lambertian {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        let color = Vec3(x, y, z);
        let texture = SolidColor::new(color);
        Self {
            albedo: Rc::new(texture),
        }
    }
}

impl From<Vec3> for Lambertian {
    fn from(color: Vec3) -> Self {
        Self::from((color.x(), color.y(), color.z()))
    }
}

impl From<(Vec3, Vec3)> for Lambertian {
    fn from((color1, color2): (Vec3, Vec3)) -> Self {
        let texture = CheckerTexture::new(color1, color2);
        Self {
            albedo: Rc::new(texture),
        }
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(color: Vec3, fuzz: f64) -> Self {
        Metal {
            albedo: color,
            fuzz: clamp(fuzz, 0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(Vec3::unit_vector(ray_in.direction()), record.normal);
        *scattered = Ray::new(
            record.p,
            reflected + self.fuzz * Vec3::rand_in_unit_sphere(),
            ray_in.time(),
        );
        *attenuation = self.albedo;
        Vec3::dot(scattered.direction(), record.normal) > 0.0
    }
}

pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Dielectric { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Vec3(1.0, 1.0, 1.0);
        let etai_over_etat = if record.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = Vec3::unit_vector(ray_in.direction());

        let cos_theta = f64::min(Vec3::dot(-unit_direction, record.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);
        if etai_over_etat * sin_theta > 1.0 {
            let reflected = Vec3::reflect(unit_direction, record.normal);
            *scattered = Ray::new(record.p, reflected, ray_in.time());
            return true;
        }

        let reflect_prob = schlick(cos_theta, etai_over_etat);
        if crate::random_double() < reflect_prob {
            let reflected = Vec3::reflect(unit_direction, record.normal);
            *scattered = Ray::new(record.p, reflected, ray_in.time());
            return true;
        }

        let refracted = refract(unit_direction, record.normal, etai_over_etat);
        *scattered = Ray::new(record.p, refracted, ray_in.time());
        true
    }
}
