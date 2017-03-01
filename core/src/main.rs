extern crate radium_ecs;
extern crate radium_graphics;

use radium_graphics::{
    GlyphSet, GlyphBatch, Tile, 
    DrawTarget, Color, Window, 
    Event, Events};


fn main() {
    // Create the window of the application
    let mut window = Window::new(800, 600);

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
        for event in window.events() {
        }

        window.clear();

        glyphbatch.flush_tiles();
        window.draw_glyphbatch(&mut glyphbatch);
    }
}