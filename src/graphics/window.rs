use graphics::GlyphBatch;

use sfml::system::Vector2f;
use sfml::window::{ContextSettings, Key, VideoMode, style};
use sfml::window::Window as SFWindow;
use sfml::window::Event as SFEvent;
use sfml::graphics::{RenderWindow, RenderTarget};
use sfml::graphics::Color as SFColor;
use std::{time, thread};

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
    _render_window:RenderWindow
}

impl Window {
    pub fn new(width:u32, height:u32) -> Window {
        let mut window = match RenderWindow::new(
            VideoMode::new(width, height, 32),
            "Radium",
            style::CLOSE,
            &ContextSettings::default()
        ) {
            Some(window) => window,
            None => panic!("Cannot create a new Render Window.")
        };

        window.set_vertical_sync_enabled(true);

        Window {
            _render_window: window
        }
    }

    pub fn clear(&mut self) {
        self._render_window.clear(&SFColor::black());
    }

    pub fn draw_glyphbatch(&mut self, glyphbatch:& GlyphBatch) {
        glyphbatch.render(&mut self._render_window);
        self._render_window.display();
    }

    pub fn events(&mut self) -> Vec<Event> {
        let mut out_events:Vec<Event> = Vec::new();
        let mut     close = false;
        for event in  self._render_window.events() {
            close =  match event {
                SFEvent::Closed => true,
                _  => {
                    out_events.push(Window::convert_event(event));
                    close
                }
            };
            
        }
        if close {
            self.close();
            return Vec::new();
        }
        out_events
    }

    pub fn wait_for_event(&mut self) -> Event {
        //can simply unwrap, as failure will be caused only by error
        loop {

            let sf_event = self._render_window.wait_event().unwrap();
            let event = Self::convert_event(sf_event);
            match event {
                Event::Close => self.close(),
                Event::None  => continue,
                _            => {}
            };
            return event;
        }
    }

    pub fn is_open(&self) -> bool {
        self._render_window.is_open()
    }

    pub fn close(&mut self) {
        self._render_window.close();
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
            Key::D => Event::KeyPress{code:'d', alt, ctrl, shift},
            Key::S => Event::KeyPress{code:'s', alt, ctrl, shift},
            Key::W => Event::KeyPress{code:'w', alt, ctrl, shift},
            _      => Event::None
        }
    }
}