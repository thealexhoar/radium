use ecs::*;
use graphics::*;
use sfml::system::Clock;
use game::action::Action;
use game::Blackboard;
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

        let mut blackboard = Blackboard::new();

        let mut camera = Camera::new(0,0,0);
        blackboard.camera = Some(camera);

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

        self.engine.load(&mut blackboard);

        self.clock.restart();
        while self.window.is_open() {
            let delta_time = self.clock.restart();
            if delta_time.as_seconds() > 1. {
                println!("Delta time: {}", delta_time.as_seconds());
            }


            if self.window.mouse_pressed(MouseButton::Left) {
                let (mouse_x, mouse_y) = self.window.mouse_pos();
                let (x,y) = glyphbatch.get_tile_from_pos(
                    mouse_x,
                    mouse_y
                );  
                println!("x: {} y: {}", x, y);
                println!("{}", 1. / delta_time.as_seconds());
            }

            self.engine.update_passive_systems(
                &mut glyphbatch,
                &mut self.window,
                &mut blackboard,
                delta_time.as_seconds()
            );

            self.window.clear();
            glyphbatch.flush_tiles();
            self.window.draw_glyphbatch(&glyphbatch);

            let events = self.window.events();

            let mut next_state = self.state;
            match self.state {
                CoreState::View               => {
                    //listen for global actions
                    //listen for selections
                    self.control_camera(
                        &mut blackboard,
                        &events
                    );
                },

                CoreState::Selected           => {
                    //listen for escaping selection mode
                    //listen for switching selection
                    //listen for actions pertaining to selected unit
                    self.control_camera(
                        &mut blackboard,
                        &events
                    );
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
                                &mut blackboard,
                                delta_time.as_seconds()
                        ),
                        None             => (true, 0)
                    };
                    if delta > 0 { 
                        self.engine.update_continuous_systems(
                            &mut blackboard,
                            delta
                        );
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

    fn control_camera(
        &mut self,
        blackboard: &mut Blackboard,
        events: &Vec<Event>
    ) {
        let camera = match blackboard.camera {
            Some(ref mut cam) => cam,
            None      => { return; }
        };
        for &event in events.iter() {
            match event {
                Event::KeyPress{code, alt, ctrl, shift} => 
                    match (code, alt, ctrl, shift) {
                        ('a', false, false, false) => camera.move_west(),
                        ('d', false, false, false) => camera.move_east(),
                        ('e', false, false, false) => camera.move_down(),
                        ('q', false, false, false) => camera.move_up(),
                        ('s', false, false, false) => camera.move_south(),
                        ('w', false, false, false) => camera.move_north(),
                        _ => { }
                },
                _ => { }
            }
        }
    }
}