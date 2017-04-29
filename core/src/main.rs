extern crate radium_ecs;
extern crate radium_graphics;

use radium_graphics::{
    GlyphSet, GlyphBatch, Tile, 
    DrawTarget, Color, Window, 
    Event, Events};


fn main() {
    let width = 1920;
    let height = 1080;
    // Create the window of the application
    let mut window = Window::new(width, height);

    let glyphset = GlyphSet::new("assets/tileset_10x10.png", 10, 10, 256);
    let mut glyphbatch:GlyphBatch = GlyphBatch::new(
        glyphset, 
        192, 108,
        width, height
    );

    glyphbatch.drawtarget.set_tiles_rect(
        Some(Tile::new(
            Some(Color::new_from_rgb(100,100,100)), 
            None, 
            138
        )),
        5, 3,
        22, 10
    );

    for i in 0..4 {
        glyphbatch.drawtarget.overlay_tile(
            Some(Tile::new(
                Some(Color::yellow()),
                None,
                208 + i
            )),
            7 + i, 8 + i
        );
    }

    glyphbatch.drawtarget.draw_string_slice(
        "Hello World!", 
        5, 2, 
        Color::black(), 
        Some(Color::cyan())
    );

    'outer: loop {
        for event in window.events() {
            match event {
                Event::Closed => {
                    window.close();
                    break 'outer;
                },
                _             => {}
            }
        }

        window.clear();

        glyphbatch.flush_tiles();
        window.draw_glyphbatch(&mut glyphbatch);
    }
}