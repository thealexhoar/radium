use ecs::*;
use graphics::*;
use game::action::*;
use game::Blackboard;
use game::Direction;
use game::graphics::*;
use game::ui::*;
use std::ops::DerefMut;


const WINDOW_WIDTH:u32   = 80;
const WINDOW_HEIGHT:u32  = 45;
const GAME_WIDTH:u32     = 55;
const GAME_HEIGHT:u32    = 31;
const CONSOLE_WIDTH:u32  = GAME_WIDTH;
const CONSOLE_HEIGHT:u32 = WINDOW_HEIGHT - GAME_HEIGHT;
const INFO_WIDTH:u32     = WINDOW_WIDTH - GAME_WIDTH;
const INFO_HEIGHT:u32    = WINDOW_HEIGHT;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CoreState {
    PlayerTurn,
    EnemyTurn,

    PlayerAction,
    EnemyAction
}

pub struct Core {
    width: u32,
    height: u32,
    blackboard: Blackboard,
    current_action: Option<Box<Action>>,
    engine: Engine,
    glyphbatch: GlyphBatch,
    mouse_interface: MouseInterface,
    state: CoreState,
    selected_entity: Option<Entity>,
    window: Window
}


impl Core {
    pub fn new(width: u32, height: u32) -> Core {
        Core {
            width,
            height,
            blackboard: Blackboard::new(),
            current_action: None,
            engine: Engine::new(),
            glyphbatch: GlyphBatch::new(
                GlyphSet::new("assets/tileset_10x10.png", 10, 10, 256),
                WINDOW_WIDTH, WINDOW_HEIGHT,
                width, height
            ),
            mouse_interface: MouseInterface::new(
                GAME_WIDTH, GAME_HEIGHT,
                INFO_WIDTH, INFO_HEIGHT
            ),
            state: CoreState::PlayerTurn,
            selected_entity: None,
            window: Window::new(width, height),
        }
    }

    pub fn init(&mut self) {
        self.blackboard.camera = Some(Camera::new(0,0,0));

        self.engine.passive_systems.push(
            Box::new(RenderSystem::new(
                GAME_WIDTH,
                GAME_HEIGHT,
                0, 0
            ))
        );
        self.engine.passive_systems.push(
            Box::new(ConsoleSystem::new(
                GAME_WIDTH,
                WINDOW_HEIGHT - GAME_HEIGHT,
                0, GAME_HEIGHT
            ))
        );
        self.engine.passive_systems.push(
            Box::new(InfoSystem::new(
                WINDOW_WIDTH - GAME_WIDTH,
                WINDOW_HEIGHT,
                GAME_WIDTH, 0
            ))
        );
    }

    pub fn run(&mut self) {



        self.engine.load(&mut self.blackboard);

        while self.window.is_open() {
            let delta_time = self.window.elapsed_time();

            self.mouse_interface.update(&self.window);

            self.engine.update_passive_systems(
                &mut self.glyphbatch,
                &mut self.window,
                &mut self.blackboard,
                delta_time
            );

            self.window.clear();
            self.glyphbatch.flush_tiles();
            self.window.draw_glyphbatch(&self.glyphbatch);

            let events = self.window.events();

            let mut next_state = self.state;
            match self.state {
                CoreState::PlayerTurn         => {
                    //listen for actions pertaining to selected unit
                    next_state = self.keyboard_control(
                        &events,
                        1
                    );
                    if self.state == next_state {
                        next_state = self.mouse_control(&events);
                    }

                    self.control_camera(
                        &events
                    );
                },

                CoreState::EnemyTurn          => {
                    //iterate through enemy controllers
                },

                CoreState::PlayerAction |
                CoreState::EnemyAction        => {
                    let (completed, delta) = match self.current_action {
                        Some(ref mut action_box) => action_box
                            .deref_mut()
                            .execute(
                                &mut self.engine.component_manager,
                                &mut self.engine.space,
                                &mut self.blackboard,
                                delta_time
                        ),
                        None => (true, 0)
                    };
                    if delta > 0 {
                        self.engine.update_continuous_systems(
                            &mut self.blackboard,
                            delta
                        );
                    }
                    if completed {
                        self.current_action = None;
                        match self.state {
                            CoreState::PlayerAction =>
                                next_state = CoreState::PlayerTurn,
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

    fn keyboard_control(
        &mut self,
        events: &Vec<Event>,
        entity: Entity
    ) -> CoreState {
        for &event in events.iter() {
            match event {
                Event::KeyPress{code, alt, ctrl, shift} => {
                    let mut action_taken = true;
                    self.current_action = match (code, alt, ctrl, shift) {
                        (Key::A, false, false, true) => Some(
                            Box::new(
                                MoveAction::new(
                                    entity,
                                    Direction::West,
                                    100 //TODO: use dynamic value
                                )
                            )
                        ),
                        (Key::D, false, false, true) => Some(
                            Box::new(
                                MoveAction::new(
                                    entity,
                                    Direction::East,
                                    100 //TODO: use dynamic value
                                )
                            )
                        ),
                        (Key::E, false, false, true) => Some(
                            Box::new(
                                MoveAction::new(
                                    entity,
                                    Direction::Down,
                                    100 //TODO: use dynamic value
                                )
                            )
                        ),
                        (Key::Q, false, false, true)  => Some(
                            Box::new(
                                MoveAction::new(
                                    entity,
                                    Direction::Up,
                                    100 //TODO: use dynamic value
                                )
                            )
                        ),
                        (Key::S, false, false, true) => Some(
                            Box::new(
                                MoveAction::new(
                                    entity,
                                    Direction::South,
                                    100 //TODO: use dynamic value
                                )
                            )
                        ),
                        (Key::W, false, false, true) => Some(
                            Box::new(
                                MoveAction::new(
                                    entity,
                                    Direction::North,
                                    100 //TODO: use dynamic value
                                )
                            )
                        ),
                        _ => { action_taken = false; None }
                    };
                    if action_taken {
                        return CoreState::PlayerAction
                    }
                },
                _ => {}
            }
        }
        CoreState::PlayerTurn
    }

    fn mouse_control(
        &mut self,
        events: &Vec<Event>
    ) -> CoreState {
        let mut action_taken = false;
        //TODO: implement mouse control
        self.state
    }

    fn control_camera(
        &mut self,
        events: &Vec<Event>
    ) {
        let camera = match self.blackboard.camera {
            Some(ref mut cam) => cam,
            None      => { return; }
        };
        for &event in events.iter() {
            match event {
                Event::KeyPress{code, alt, ctrl, shift} => {
                    match (code, alt, ctrl, shift) {
                        (Key::A, false, false, false) => camera.move_west(),
                        (Key::D, false, false, false) => camera.move_east(),
                        (Key::E, false, false, false) => camera.move_down(),
                        (Key::Q, false, false, false) => camera.move_up(),
                        (Key::S, false, false, false) => camera.move_south(),
                        (Key::W, false, false, false) => camera.move_north(),
                        _ => { }
                    }
                },
                _ => { }
            }
        }
    }

}