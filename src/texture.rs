use crate::color::Color;
use crate::vector::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct CheckerTexture {
    odd: Color,
    even: Color
}


impl CheckerTexture {
    pub fn new(c1: Color, c2: Color) -> CheckerTexture {
        CheckerTexture {odd: c1, even: c2}
    }
    pub fn get_color(&self, coords: &TextureCoords, normal: &Vec3) -> Color {
        let sines = f64::sin(10. * normal.x()) * f64::sin(10. * normal.y()) * f64::sin(10. * normal.z());
        if sines < 0. {
            return self.odd
        } else {
            return self.even
        }
    }
}

pub struct TextureCoords {
    pub u: f64,
    pub v: f64
}

