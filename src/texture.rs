use crate::vec3::{Color, Vec3};
use std::rc::Rc;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Color;
}

pub struct SolidColor {
    color: Color
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        SolidColor { color }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Vec3) -> Color {
        self.color
    }
}

pub struct CheckerTexture {
    odd: Rc<dyn Texture>,
    even: Rc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(odd: Color, even: Color) -> Self {
        let odd = SolidColor::new(odd);
        let even = SolidColor::new(even);
        CheckerTexture {
            odd: Rc::new(odd),
            even: Rc::new(even),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Color {
        let sines = f64::sin(10.0 * p.x()) * f64::sin(10.0 * p.y()) * f64::sin(10.0 * p.z());
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
    
}

