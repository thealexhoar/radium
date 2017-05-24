use ecs::*;
use graphics::*;
use sfml::system::Clock;
use game::graphics::*;


pub struct Core {
    width: u32,
    height: u32,
    window: Window,
    engine: Engine, 
    clock: Clock
}

impl Core {
    pub fn new(width: u32, height: u32) -> Core {
        Core {
            width,
            height,
            window: Window::new(width, height),
            engine: Engine::new(),
            clock: Clock::start()
        }
    }

    pub fn init(&mut self) {

    }

    pub fn run(&mut self) {
        let mut glyphbatch = GlyphBatch::new(
            GlyphSet::new("assets/tileset_10x10.png", 10, 10, 256),
            80, 45,
            self.width, self.height
        );
        glyphbatch.drawtarget.set_tiles_rect(
            Some(Tile::new(
                Some(Color::new_from_rgb(40,40,40)), 
                None, 
                '.' as u32//138
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

        self.engine.passive_systems.push(
            Box::new(RenderSystem::new(45, 45, 0, 0, glyphbatch))
        );

        self.engine.load();

        self.clock.restart();
        while self.window.is_open() {
            let delta_time = self.clock.restart();
            self.engine.update(&mut self.window, delta_time.as_seconds());
            //clear event queue and check for closing event
            //self.window.events();
        }
    }
}