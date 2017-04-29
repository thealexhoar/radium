
use graphics::{GlyphSet, GlyphBatch, Tile, Color, Window,  Event};

pub struct Core {
    width:u32,
    height:u32,
    window:Window,
    glyphbatch:GlyphBatch
}

impl Core {
    pub fn new(width: u32, height: u32) -> Core {
        Core {
            width,
            height,
            window: Window::new(width, height),
            glyphbatch: GlyphBatch::new(
                GlyphSet::new("assets/tileset_10x10.png", 10, 10, 256),
                80, 45,
                width, height
            )
        }
    }

    pub fn init(&mut self) {
        self.glyphbatch.drawtarget.set_tiles_rect(
            Some(Tile::new(
                Some(Color::new_from_rgb(100,100,100)), 
                None, 
                '.' as u32//138
            )),
            5, 3,
            22, 10
        );

        for i in 0..4 {
            self.glyphbatch.drawtarget.overlay_tile(
                Some(Tile::new(
                    Some(Color::yellow()),
                    None,
                    208 + i
                )),
                7 + i, 8 + i
            );
        }

        self.glyphbatch.drawtarget.draw_string_slice(
            "Hello World!", 
            5, 2, 
            Color::black(), 
            Some(Color::cyan())
        );
    }

    pub fn run(&mut self) {
        'outer: loop {
            for event in self.window.events() {
                match event {
                    Event::Closed => {
                        self.window.close();
                        break 'outer;
                    },
                    _             => {}
                }
            }

            self.window.clear();

            self.glyphbatch.flush_tiles();
            self.window.draw_glyphbatch(&mut self.glyphbatch);
        }
    }
}