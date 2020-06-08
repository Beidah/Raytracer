use crate::{
    clamp,
    noise::Perlin,
    vec3::{Color, Vec3},
};
use image::{DynamicImage, GenericImageView};
use std::{path::Path, rc::Rc};

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Color;
}

pub struct SolidColor {
    color: Color,
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

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
    color: Color,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        NoiseTexture {
            scale,
            ..Self::default()
        }
    }

    pub fn with_color(scale: f64, color: Color) -> Self {
        NoiseTexture {
            scale,
            color,
            ..Self::default()
        }
    }

    pub fn set_scale(&mut self, scale: f64) {
        self.scale = scale;
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Vec3) -> Color {
        return self.color * (self.scale * p.z() + 10.0 * self.noise.turb(p)).sin().abs();
    }
}

impl Default for NoiseTexture {
    fn default() -> Self {
        NoiseTexture {
            scale: 1.0,
            color: Vec3(1.0, 1.0, 1.0),
            noise: Default::default(),
        }
    }
}

pub struct ImageTexture {
    image: DynamicImage,
}

impl ImageTexture {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let image = image::open(path).unwrap();
        Self { image }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: Vec3) -> Color {
        let color_scale: f64 = 1.0 / 255.0;

        let u = clamp(u, 0.0, 1.0);
        let v = 1.0 - clamp(v, 0.0, 1.0);

        let (width, height) = self.image.dimensions();

        let x = (u * width as f64) as u32;
        let y = (v * height as f64) as u32;

        let pixel = self.image.get_pixel(x, y);

        let r = pixel[0] as f64 * color_scale;
        let g = pixel[1] as f64 * color_scale;
        let b = pixel[2] as f64 * color_scale;

        Vec3(r, g, b)
    }
}
