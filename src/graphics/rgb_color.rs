extern crate sfml;

use std::cmp::max;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RGBColor {
    pub r:u8,
    pub g:u8,
    pub b:u8,
    pub a:u8
}

impl RGBColor {
    pub fn new_from_rgba(r:u8, g:u8, b:u8, a:u8) -> Self {
        Self {r,g,b,a}
    }

    pub fn new_from_rgb(r:u8, g:u8, b:u8) -> Self {
        Self::new_from_rgba(r,g,b,255)
    }

    pub fn new_from_rgba_f(r:f32, g:f32, b:f32, a:f32) -> Self {
        let r8 = (r * 255.) as u8;
        let g8 = (g * 255.) as u8;
        let b8 =  (b * 255.) as u8;
        let a8 = (a * 255.) as u8;
        Self::new_from_rgba(r8, g8, b8, a8)
    }

    pub fn new_from_rgb_f(r:f32, g:f32, b:f32) -> Self {
        Self::new_from_rgba_f(r,g,b,1.)
    }

    pub fn pow_stepdown(&self, step: u32, pow:u32) -> Self {
        let step_val = (step + 1).pow(pow);
        Self::new_from_rgb(
            ((self.r as u32) / step_val) as u8,
            ((self.g as u32) / step_val) as u8,
            ((self.b as u32) / step_val) as u8
        )
    }

    pub fn frac_stepdown(&self, step:u32, frac:u32) -> Self {
        let adj_step = max(step, frac);

        Self::new_from_rgb(
            (((self.r as u32) * adj_step) / frac) as u8,
            (((self.g as u32) * adj_step) / frac) as u8,
            (((self.b as u32) * adj_step) / frac) as u8
        )
    }

    pub fn lin_stepdown(&self, step:u32, mag:u32) -> Self {
        Self::new_from_rgb(
            self.r - max(self.r as u32, step * mag) as u8,
            self.g - max(self.g as u32, step * mag) as u8,
            self.b - max(self.b as u32, step * mag) as u8
        )
    }

    pub fn grayscale(&self) -> Self {
        let sum = (self.r as u32) + (self.g as u32) + (self.b as u32);
        let out = (sum / 3) as u8;
        Self::new_from_rgb(out, out, out)
    }

    pub fn grayscale_pow_stepdown(&self, step: u32, pow:u32) -> Self {
        let sum = (self.r as u32) + (self.g as u32) + (self.b as u32);
        let out = (sum / 3) as u8;
        Self::new_from_rgb(out, out, out).pow_stepdown(step, pow)
    }

    pub fn white() -> Self {
        Self::new_from_rgb(255,255,255)
    }

    pub fn gray() -> Self {
        Self::new_from_rgb(127,127,127)
    }

    pub fn black() -> Self {
        Self::new_from_rgb(0,0,0)
    }

    pub fn clear() -> Self {
        Self::new_from_rgba(0,0,0,0)
    }

    pub fn red() -> Self {
        Self::new_from_rgb(255,0,0)
    }

    pub fn green() -> Self {
        Self::new_from_rgb(0,255,0)
    }

    pub fn blue() -> Self {
        Self::new_from_rgb(0,0,255)
    }

    pub fn yellow() -> Self {
        Self::new_from_rgb(255,255,0)
    }

    pub fn magenta() -> Self {
        Self::new_from_rgb(255,0,255)
    }

    pub fn cyan() -> Self {
        Self::new_from_rgb(0,255,255)
    }
}