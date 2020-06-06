use crate::vec3::Vec3;

#[derive(Debug, Default, Copy, Clone)]
pub struct Ray {
    origin: Vec3,
    dir: Vec3,
    time: f64
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3, time: f64) -> Self {
        Self { origin, dir, time }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.dir * t
    }

    pub fn time(&self) -> f64 {
        self.time
    }
}
