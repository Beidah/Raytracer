const POINT_COUNT: usize = 256;

use crate::vec3::Vec3;
use rand::prelude::*;

pub struct Perlin {
    rand_vec: Vec<Vec3>,
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

fn perlin_generate_perm() -> [usize; POINT_COUNT] {
    let mut p = [0; POINT_COUNT];

    for (i, p) in p.iter_mut().enumerate() {
        *p = i;
    }

    let mut rng = thread_rng();

    p.shuffle(&mut rng);

    p
}

fn trilinear_interp(c: Vec<Vec<Vec<Vec3>>>, u: f64, v: f64, w: f64) -> f64 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let mut accum = 0.0;

    for (i, arr) in c.iter().enumerate() {
        for (j, arr) in arr.iter().enumerate() {
            for (k, val) in arr.iter().enumerate() {
                let i = i as f64;
                let j = j as f64;
                let k = k as f64;

                let weight_v = Vec3(u - i, v - j, w - k);
                accum += (i * uu + (1.0 - i) * (1.0 - uu))
                    * (j * vv + (1.0 - j) * (1.0 - vv))
                    * (k * ww + (1.0 - k) * (1.0 - ww))
                    * Vec3::dot(*val, weight_v);
            }
        }
    }

    accum
}

impl Perlin {
    pub fn new() -> Self {
        let mut rand_vec = Vec::with_capacity(POINT_COUNT);
        for _ in 0..POINT_COUNT {
            rand_vec.push(Vec3::unit_vector(Vec3::rand_with_range(-1.0, 1.0)));
        }

        let perm_x = perlin_generate_perm();
        let perm_y = perlin_generate_perm();
        let perm_z = perlin_generate_perm();

        Perlin {
            rand_vec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: Vec3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();
        let i = p.x().floor() as usize;
        let j = p.y().floor() as usize;
        let k = p.z().floor() as usize;
        let mut c: Vec<Vec<Vec<Vec3>>> = vec![vec![vec![Vec3::default(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.rand_vec[self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255]];
                }
            }
        }

        trilinear_interp(c, u, v, w)
    }

    pub fn turb(&self, point: Vec3) -> f64 {
        self.turb_with_depth(point, 7)
    }

    pub fn turb_with_depth(&self, mut point: Vec3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(point);
            weight *= 0.5;
            point *= 2.0;
        }

        accum.abs()
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}
