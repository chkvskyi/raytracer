use crate::color::Color;
use crate::vector::Vec3;
use noise::{Perlin, NoiseFn};

#[derive(Copy, Clone, Debug)]
pub struct CheckerTexture {
    odd: Color,
    even: Color
}


impl CheckerTexture {
    pub fn new(c1: Color, c2: Color) -> CheckerTexture {
        CheckerTexture {odd: c1, even: c2}
    }
    pub fn get_color(&self, coords: &TextureCoords, ray_point: &Vec3) -> Color {
        let sines = f64::sin(10. * ray_point.x()) * f64::sin(10. * ray_point.y()) * f64::sin(10. * ray_point.z());
        if sines < 0. {
            return self.odd
        } else {
            return self.even
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct NoiseTexture {
    noise: Perlin
}

impl NoiseTexture {
    pub fn new() -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::new()
        }
    }

    pub fn get_color(&self, _coords: &TextureCoords, ray_point: &Vec3) -> Color {
        let gray: f64 = self.noise.get([ray_point.x(), ray_point.y(), ray_point.z()]);
        Color::gray(((gray + 1.) / 2.) as f32)
    }
}

fn turbulance(point: &Vec3, depth: u8, noise_ref: &Perlin) -> f64 {
    let mut acc = 0.;
    let mut weight: f64 = 1.;
    let mut point = point.as_arr();

    for _ in 0..depth {
        acc += weight * noise_ref.get(point);
        weight *= 0.5;
        point[0] = point[0] * 2.;
        point[1] = point[1] * 2.;
        point[2] = point[2] * 2.;
    }

    acc
}

pub struct TextureCoords {
    pub u: f64,
    pub v: f64
}

