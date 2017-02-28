extern crate sfml;

pub struct Color {
    pub r:u8,
    pub g:u8,
    pub b:u8,
    pub a:u8
}

impl Copy for Color {}
impl Clone for Color {
    fn clone(&self) -> Color {
        *self
    }
}

impl Color {
    pub fn new_from_rgba(r:u8, g:u8, b:u8, a:u8) -> Color {
        Color {
            r:r,
            g:g,
            b:b,
            a:a
        }
    }

    pub fn new_from_rgb(r:u8, g:u8, b:u8) -> Color{
        Color::new_from_rgba(r,g,b,255)
    }

    pub fn new_from_rgba_f(r:f32, g:f32, b:f32, a:f32) -> Color {
        let r8 = (r * 255.) as u8;
        let g8 = (g * 255.) as u8;
        let b8 =  (b * 255.) as u8;
        let a8 = (a * 255.) as u8;
        Color::new_from_rgba(r8, g8, b8, a8)
    }

    pub fn new_from_rgb_f(r:f32, g:f32, b:f32) -> Color {
        Color::new_from_rgba_f(r,g,b,1.)
    }


    pub fn white() -> Color {
        Color::new_from_rgb(255,255,255)
    }

    pub fn black() -> Color {
        Color::new_from_rgb(0,0,0)
    }

    pub fn clear() -> Color {
        Color::new_from_rgba(0,0,0,0)
    }

    pub fn red() -> Color {
        Color::new_from_rgb(255,0,0)
    }

    pub fn green() -> Color {
        Color::new_from_rgb(0,255,0)
    }

    pub fn blue() -> Color {
        Color::new_from_rgb(0,0,255)
    }

    pub fn yellow() -> Color {
        Color::new_from_rgb(255,255,0)
    }

    pub fn magenta() -> Color {
        Color::new_from_rgb(255,0,255)
    }

    pub fn cyan() -> Color {
        Color::new_from_rgb(0,255,255)
    }
}