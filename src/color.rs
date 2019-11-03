use image::Rgba;
use std::ops::{Add, Mul, Div};

const GAMMA: f32 = 2.2;

fn gamma_encode(linear: f32) -> f32 {
    linear.powf(1.0 / GAMMA)
}

#[derive(Copy, Clone, Debug)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32
}
impl Color {
    pub fn clamp(&self) -> Color {
        Color {
            r: self.r.min(1.0).max(0.0),
            g: self.g.min(1.0).max(0.0),
            b: self.b.min(1.0).max(0.0),
        }
    }

    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba([
            (gamma_encode(self.r) * 255.) as u8,
            (gamma_encode(self.g) * 255.) as u8,
            (gamma_encode(self.b) * 255.) as u8,
            255. as u8
            ])
    }

    pub fn black() -> Color {
        Color {
            r: 0.,
            g: 0.,
            b: 0.,
        }
    }

    pub fn white() -> Color {
        Color {
            r: 1.,
            g: 1.,
            b: 1.
        }
    }

    pub fn red() -> Color {
        Color {
            r: 1.,
            g: 0.,
            b: 0.
        }
    }

    pub fn green() -> Color {
        Color {
            r: 0.,
            g: 1.,
            b: 0.
        }
    }

    pub fn blue() -> Color {
        Color {
            r: 0.,
            g: 0.,
            b: 1.
        }
    }

    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color {
            r: r,
            g: g,
            b: b
        }
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, other: f32) -> Color {
        Color {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other
        }
    }
}

impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, color: Color) -> Color {
        Color {
            r: color.r * self,
            g: color.g * self,
            b: color.b * self
        }
    }
}

impl Div<f32> for Color {
    type Output = Color;

    fn div(self, other: f32) -> Color {
        Color {
            r: self.r / other,
            g: self.g / other,
            b: self.b / other
        }
    }
}