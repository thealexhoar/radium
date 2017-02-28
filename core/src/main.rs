extern crate sfml;
extern crate radium_ecs;
extern crate radium_graphics;

use radium_graphics::{GlyphSet, GlyphBatch, Tile, DrawTarget, Color};

use sfml::system::Vector2f;
use sfml::window::{ContextSettings, VideoMode, event, window_style};
use sfml::graphics::{RenderWindow, RenderTarget};
use sfml::graphics::Color as SFColor;

fn main() {
    // Create the window of the application
    let mut window = match RenderWindow::new(VideoMode::new_init(800, 600, 32),
                                             "Radium",
                                             window_style::CLOSE,
                                             &ContextSettings::default()) {
        Some(window) => window,
        None => panic!("Cannot create a new Render Window.")
    };

    window.set_vertical_sync_enabled(true);

    let glyphset = GlyphSet::new("assets/tileset_10x10.png", 10, 10, 256);
    let mut glyphbatch:GlyphBatch = GlyphBatch::new(
        glyphset, 
        40, 30, 
        800, 600
    );

    glyphbatch.drawtarget.set_tiles_rect(
        Some(Tile::new(
            Some(Color::white()), 
            Some(Color::new_from_rgb(10,10,55)), 
            225
        )), 
        5, 3, 
        22, 10
        );

    glyphbatch.drawtarget.draw_string_slice(
        "Hello World!", 
        5, 2, 
        Color::black(), 
        Some(Color::cyan())
    );

    while window.is_open() {
        // Handle events
        for event in window.events() {
            match event {
                event::Closed => window.close(),
                _             => {/* do nothing */}
            }
        }

        // Clear the window
        window.clear(&SFColor::black());

        glyphbatch.flush_tiles();
        glyphbatch.render(&mut window);

        // Display things on screen
        window.display()
    }
}