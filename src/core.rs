use ecs::*;
use graphics::*;
use sfml::system::Clock;
use game::action::Action;
use game::graphics::*;
use game::ui::*;
use std::ops::DerefMut;

#[derive(Clone, Copy)]
pub enum CoreState {
    View,
    Selected,
    EnemyTurn,

    ViewAction,
    SelectedAction,
    EnemyAction
}


pub struct Core {
    width: u32,
    height: u32,
    window: Window,
    engine: Engine, 
    clock: Clock,
    state: CoreState,
    current_action: Option<Box<Action>>
}

impl Core {
    pub fn new(width: u32, height: u32) -> Core {
        Core {
            width,
            height,
            window: Window::new(width, height),
            engine: Engine::new(),
            clock: Clock::start(),
            state: CoreState::View,
            current_action: None
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
            self.engine.update_passive_systems(
                &mut glyphbatch,
                &mut self.window,
                delta_time.as_seconds()
            );

            self.window.clear();
            glyphbatch.flush_tiles();
            self.window.draw_glyphbatch(&glyphbatch);

            let mut next_state = self.state;
            match self.state {
                CoreState::View               => {
                    //listen for actions
                },

                CoreState::Selected           => {
                    //listen for actions
                },

                CoreState::EnemyTurn          => {
                    //iterate through enemy controllers
                },
                
                CoreState::ViewAction |
                CoreState::SelectedAction |
                CoreState::EnemyAction        => {
                    let (completed, delta) = match self.current_action {
                        Some(ref mut action_box) => action_box
                            .deref_mut()
                            .execute(
                                &mut self.engine.component_manager,
                                &mut self.engine.space,
                                delta_time.as_seconds()
                        ),
                        None             => (true, 0)
                    };
                    if delta > 0 { 
                        self.engine.update_continuous_systems(delta);
                    }
                    if completed {
                        match self.state {
                            CoreState::ViewAction     =>
                                next_state = CoreState::View,
                            CoreState::SelectedAction =>
                                next_state = CoreState::Selected,
                            CoreState::EnemyAction    =>
                                next_state = CoreState::EnemyTurn,
                            _ => {}
                        }
                    }
                },
                
                _ => {}
            };

            self.state = next_state;

            self.window.update_event_queue();
        }
    }
}