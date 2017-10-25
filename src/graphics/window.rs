use graphics::GlyphBatch;
use graphics::Key;

use sfml::window::mouse;
use sfml::window::{ContextSettings, VideoMode, style};
use sfml::window::Key as SFKey;
use sfml::window::Event as SFEvent;
use sfml::graphics::{RenderWindow, RenderTarget};
use sfml::graphics::Color as SFColor;
use sfml::system::Clock;

pub enum MouseButton {
    Left,
    Right
}

pub enum Event {
    Close,
    KeyPress {
        code: Key,
        alt: bool,
        ctrl: bool,
        shift: bool
    },
    None
}
impl Copy for Event {}
impl Clone for Event {
    fn clone(&self) -> Event {
        *self
    }
}

pub struct Window {
    _render_window: RenderWindow,
    _events: Vec<Event>,
    _clock: Clock
}

impl Window {
    pub fn new(width:u32, height:u32) -> Window {
        let mut window = RenderWindow::new(
            VideoMode::new(width, height, 32),
            "Radium",
            style::CLOSE,
            &ContextSettings::default()
        );

        window.set_vertical_sync_enabled(true);
        window.set_key_repeat_enabled(false);

        Window {
            _render_window: window,
            _events: Vec::new(),
            _clock: Clock::default()
        }
    }

    pub fn elapsed_time(&mut self) -> f32 {
        self._clock.restart().as_seconds()
    }

    pub fn clear(&mut self) {
        self._render_window.clear(&SFColor::black());
    }

    pub fn draw_glyph_batch(&mut self, glyph_batch:& GlyphBatch) {
        glyph_batch.render(&mut self._render_window);
        self._render_window.display();
    }

    pub fn update_event_queue(&mut self) {
        self._events = Vec::new();
        let mut close = false;
        for event in  self._render_window.events() {
            close =  match event {
                SFEvent::Closed => true,
                _  => {
                    self._events.push(Window::convert_event(event));
                    close
                }
            };
        }
        if close {
            self.close();
        }
    }

    pub fn events(&self) -> Vec<Event> {
        let mut out_vec = Vec::with_capacity(self._events.len());
        for &event in self._events.iter() {
            out_vec.push(event);
        }
        out_vec
    }

    pub fn is_open(&self) -> bool {
        self._render_window.is_open()
    }

    pub fn close(&mut self) {
        self._render_window.close();
    }

    pub fn mouse_pressed(&self, button:MouseButton) -> bool {
        match button {
            MouseButton::Left => {
                mouse::Button::Left.is_pressed()
            },
            MouseButton::Right => {
                mouse::Button::Right.is_pressed()
            }
        }
    }

    pub fn mouse_pos(&self) -> (Option<u32>, Option<u32>) {
        let mouse_point = mouse::desktop_position();
        let window_point = self._render_window.position();

        let x = match mouse_point.x >= window_point.x {
            true  => Some((mouse_point.x - window_point.x) as u32),
            false => None
        };
        let y = match mouse_point.y >= window_point.y {
            true  => Some((mouse_point.y - window_point.y) as u32),
            false => None
        };

        (x,y)
    }


    fn convert_event(sf_event:SFEvent) -> Event {
        match sf_event {
            SFEvent::Closed => Event::Close,
            SFEvent::KeyPressed {code, alt, ctrl, shift, system }
                            => Self::convert_key_event(code, alt, ctrl, shift),
            _               => Event::None
        }
    }

    fn convert_key_event (
        code:SFKey,
        alt: bool, ctrl: bool, shift: bool
    ) -> Event {
        match code {
            SFKey::A => Event::KeyPress{code:Key::A, alt, ctrl, shift},
            SFKey::B => Event::KeyPress{code:Key::B, alt, ctrl, shift},
            SFKey::C => Event::KeyPress{code:Key::C, alt, ctrl, shift},
            SFKey::D => Event::KeyPress{code:Key::D, alt, ctrl, shift},
            SFKey::E => Event::KeyPress{code:Key::E, alt, ctrl, shift},
            SFKey::F => Event::KeyPress{code:Key::F, alt, ctrl, shift},
            SFKey::G => Event::KeyPress{code:Key::G, alt, ctrl, shift},
            SFKey::H => Event::KeyPress{code:Key::H, alt, ctrl, shift},
            SFKey::I => Event::KeyPress{code:Key::I, alt, ctrl, shift},
            SFKey::J => Event::KeyPress{code:Key::J, alt, ctrl, shift},
            SFKey::K => Event::KeyPress{code:Key::K, alt, ctrl, shift},
            SFKey::L => Event::KeyPress{code:Key::L, alt, ctrl, shift},
            SFKey::M => Event::KeyPress{code:Key::M, alt, ctrl, shift},
            SFKey::N => Event::KeyPress{code:Key::N, alt, ctrl, shift},
            SFKey::O => Event::KeyPress{code:Key::O, alt, ctrl, shift},
            SFKey::P => Event::KeyPress{code:Key::P, alt, ctrl, shift},
            SFKey::Q => Event::KeyPress{code:Key::Q, alt, ctrl, shift},
            SFKey::R => Event::KeyPress{code:Key::R, alt, ctrl, shift},
            SFKey::S => Event::KeyPress{code:Key::S, alt, ctrl, shift},
            SFKey::T => Event::KeyPress{code:Key::T, alt, ctrl, shift},
            SFKey::U => Event::KeyPress{code:Key::U, alt, ctrl, shift},
            SFKey::V => Event::KeyPress{code:Key::V, alt, ctrl, shift},
            SFKey::W => Event::KeyPress{code:Key::W, alt, ctrl, shift},
            SFKey::X => Event::KeyPress{code:Key::X, alt, ctrl, shift},
            SFKey::Y => Event::KeyPress{code:Key::Y, alt, ctrl, shift},
            SFKey::Z => Event::KeyPress{code:Key::Z, alt, ctrl, shift},
            SFKey::Num0 => Event::KeyPress{code:Key::Num0, alt, ctrl, shift},
            SFKey::Num1 => Event::KeyPress{code:Key::Num1, alt, ctrl, shift},
            SFKey::Num2 => Event::KeyPress{code:Key::Num2, alt, ctrl, shift},
            SFKey::Num3 => Event::KeyPress{code:Key::Num3, alt, ctrl, shift},
            SFKey::Num4 => Event::KeyPress{code:Key::Num4, alt, ctrl, shift},
            SFKey::Num5 => Event::KeyPress{code:Key::Num5, alt, ctrl, shift},
            SFKey::Num6 => Event::KeyPress{code:Key::Num6, alt, ctrl, shift},
            SFKey::Num7 => Event::KeyPress{code:Key::Num7, alt, ctrl, shift},
            SFKey::Num8 => Event::KeyPress{code:Key::Num8, alt, ctrl, shift},
            SFKey::Num9 => Event::KeyPress{code:Key::Num9, alt, ctrl, shift},
            SFKey::Space => Event::KeyPress{code:Key::Space, alt, ctrl, shift},
            SFKey::Tab => Event::KeyPress{code:Key::Tab, alt, ctrl, shift},
            SFKey::Return =>
                Event::KeyPress{code:Key::Return, alt, ctrl, shift},
            SFKey::Escape => Event::KeyPress{code:Key::Escape, alt, ctrl, shift},
            _      => Event::None
        }
    }
}