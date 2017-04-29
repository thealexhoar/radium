use graphics::GlyphBatch;

use sfml::system::Vector2f;
use sfml::window::{ContextSettings, VideoMode, style};
use sfml::window::Event as SFEvent;
use sfml::graphics::{RenderWindow, RenderTarget};
use sfml::graphics::Color as SFColor;

pub enum Event {
    Closed,
    None
}
impl Copy for Event {}
impl Clone for Event {
    fn clone(&self) -> Event {
        *self
    }
}

pub struct Events {
    _events:Vec<Event>
}

impl Events {
    pub fn new(mut events:Vec<Event>) -> Events {
        events.reverse();
        Events {
            _events: events
        }
    }
}

impl Iterator for Events {
    type Item = Event;

    fn next(&mut self) -> Option<Event> {
        match self._events.len() {
            0 => None,
            _ => {
                self._events.pop()
            }
        }
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

    pub fn draw_glyphbatch(&mut self, glyphbatch:&mut GlyphBatch) {
        glyphbatch.render(&mut self._render_window);
        self._render_window.display();
    }

    pub fn events(&mut self) -> Events {
        let mut out_events:Vec<Event> = Vec::new();
        for event in  self._render_window.events() {
            out_events.push(Window::convert_event(event));
        }
        Events::new(out_events)
    }

    pub fn is_open(&self) -> bool {
        self._render_window.is_open()
    }

    pub fn close(&mut self) {
        self._render_window.close();
    }


    fn convert_event(sf_event:SFEvent) -> Event {
        match sf_event {
            SFEvent::Closed => Event::Closed,
            _             => Event::None
        }
    }
}