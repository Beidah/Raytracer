use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{refract, Vec3};

use crate::clamp;

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
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian {
            albedo
        }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction = record.normal + Vec3::rand_unit_vector();
        *scattered = Ray::new(record.p, scatter_direction);
        *attenuation = self.albedo;
        true
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
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
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
            *scattered = Ray::new(record.p, reflected);
            return true;
        }

        let reflect_prob = schlick(cos_theta, etai_over_etat);
        if crate::random_double() < reflect_prob {
            let reflected = Vec3::reflect(unit_direction, record.normal);
            *scattered = Ray::new(record.p, reflected);
            return true;
        }

        let refracted = refract(unit_direction, record.normal, etai_over_etat);
        *scattered = Ray::new(record.p, refracted);
        true
    }
}