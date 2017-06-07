use graphics::GlyphBatch;

use sfml::window::mouse;
use sfml::system::Vector2f;
use sfml::window::{ContextSettings, Key, VideoMode, style};
use sfml::window::Window as SFWindow;
use sfml::window::Event as SFEvent;
use sfml::graphics::{RenderWindow, RenderTarget};
use sfml::graphics::Color as SFColor;
use std::cmp::max;
use std::{time, thread};

pub enum MouseButton {
    Left,
    Right
}

pub enum Event {
    Close,
    KeyPress {
        code: char,
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
    _events: Vec<Event>
}

impl Window {
    pub fn new(width:u32, height:u32) -> Window {
        let mut window = match RenderWindow::new(
            VideoMode::new(width, height, 32),
            "Radium",
            style::NONE,
            &ContextSettings::default()
        ) {
            Some(window) => window,
            None => panic!("Cannot create a new Render Window.")
        };

        window.set_vertical_sync_enabled(true);
        window.set_key_repeat_enabled(false);

        Window {
            _render_window: window,
            _events: Vec::new()
        }
    }

    pub fn clear(&mut self) {
        self._render_window.clear(&SFColor::black());
    }

    pub fn draw_glyphbatch(&mut self, glyphbatch:& GlyphBatch) {
        glyphbatch.render(&mut self._render_window);
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

    pub fn mouse_pos(&self) -> (u32, u32) {
        let mouse_point = mouse::desktop_position();
        let window_point = self._render_window.position();

        let x = max(mouse_point.x - window_point.x, 0) as u32;
        let y = max(mouse_point.y - window_point.y, 0) as u32;
        
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
        code:Key, 
        alt: bool, ctrl: bool, shift: bool
    ) -> Event {
        match code {
            Key::A => Event::KeyPress{code:'a', alt, ctrl, shift},
            Key::B => Event::KeyPress{code:'b', alt, ctrl, shift},
            Key::C => Event::KeyPress{code:'c', alt, ctrl, shift},
            Key::D => Event::KeyPress{code:'d', alt, ctrl, shift},
            Key::E => Event::KeyPress{code:'e', alt, ctrl, shift},
            Key::F => Event::KeyPress{code:'f', alt, ctrl, shift},
            Key::G => Event::KeyPress{code:'g', alt, ctrl, shift},
            Key::H => Event::KeyPress{code:'h', alt, ctrl, shift},
            Key::I => Event::KeyPress{code:'i', alt, ctrl, shift},
            Key::J => Event::KeyPress{code:'j', alt, ctrl, shift},
            Key::K => Event::KeyPress{code:'k', alt, ctrl, shift},
            Key::L => Event::KeyPress{code:'l', alt, ctrl, shift},
            Key::M => Event::KeyPress{code:'m', alt, ctrl, shift},
            Key::N => Event::KeyPress{code:'n', alt, ctrl, shift},
            Key::O => Event::KeyPress{code:'o', alt, ctrl, shift},
            Key::P => Event::KeyPress{code:'p', alt, ctrl, shift},
            Key::Q => Event::KeyPress{code:'q', alt, ctrl, shift},
            Key::R => Event::KeyPress{code:'r', alt, ctrl, shift},
            Key::S => Event::KeyPress{code:'s', alt, ctrl, shift},
            Key::T => Event::KeyPress{code:'t', alt, ctrl, shift},
            Key::U => Event::KeyPress{code:'u', alt, ctrl, shift},
            Key::V => Event::KeyPress{code:'v', alt, ctrl, shift},
            Key::W => Event::KeyPress{code:'w', alt, ctrl, shift},
            Key::X => Event::KeyPress{code:'x', alt, ctrl, shift},
            Key::Y => Event::KeyPress{code:'y', alt, ctrl, shift},
            Key::Z => Event::KeyPress{code:'z', alt, ctrl, shift},
            _      => Event::None
        }
    }
}