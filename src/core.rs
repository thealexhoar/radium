use ecs::*;
use graphics::*;
use sfml::system::Clock;
use game::graphics::*;
use game::ui::*;


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
        let window_width = 80;
        let window_height = 45;
        let game_console_width = 55;
        let game_console_height = 31;

        let mut glyphbatch = GlyphBatch::new(
            GlyphSet::new("assets/tileset_10x10.png", 10, 10, 256),
            window_width, window_height,
            self.width, self.height
        );
        self.engine.passive_systems.push(
            Box::new(RenderSystem::new(
                game_console_width,
                game_console_height,
                0, 0
            ))
        );
        self.engine.passive_systems.push(
            Box::new(ConsoleSystem::new(
                game_console_width,
                window_height - game_console_height,
                0, game_console_height
            ))
        );
        self.engine.passive_systems.push(
            Box::new(InfoSystem::new(
                window_width - game_console_width,
                window_height,
                game_console_width, 0
            ))
        );

        self.engine.load();

        self.clock.restart();
        while self.window.is_open() {
            let delta_time = self.clock.restart();
            self.engine.update(
                &mut glyphbatch,
                &mut self.window,
                delta_time.as_seconds()
            );
            //clear event queue and check for closing event
            //self.window.events();
        }
    }
}