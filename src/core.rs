use ecs::*;
use graphics::*;
use game::action::*;
use game::components::ColliderComponent;
use game::Blackboard;
use game::Direction;
use game::graphics::*;
use game::ui::*;
use std::ops::DerefMut;
use util::Point;


const WINDOW_WIDTH:u32   = 80;
const WINDOW_HEIGHT:u32  = 45;
const GAME_WIDTH:u32     = 55;
const GAME_HEIGHT:u32    = 31;
const CONSOLE_WIDTH:u32  = GAME_WIDTH;
const CONSOLE_HEIGHT:u32 = WINDOW_HEIGHT - GAME_HEIGHT;
const INFO_WIDTH:u32     = WINDOW_WIDTH - GAME_WIDTH;
const INFO_HEIGHT:u32    = WINDOW_HEIGHT;

const TURN_MAX_TIME:u32 = 300;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CoreState {
    PlayerTurn,
    EnemyTurn,

    PlayerAction,
    EnemyAction
}

pub struct Core {
    pub width: u32,
    pub height: u32,
    pub blackboard: Blackboard,
    pub current_action: Option<Box<Action>>,
    pub engine: Engine,
    pub glyphbatch: GlyphBatch,
    pub mouse_interface: MouseInterface,
    pub scheduler: Scheduler,
    pub state: CoreState,
    pub window: Window
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
                GlyphSet::new("assets/tileset_16x16.png", 16, 16, 1024),
                WINDOW_WIDTH, WINDOW_HEIGHT,
                width, height
            ),
            mouse_interface: MouseInterface::new(
                GAME_WIDTH, GAME_HEIGHT,
                INFO_WIDTH, INFO_HEIGHT
            ),
            scheduler: Scheduler::new(),
            state: CoreState::PlayerTurn,
            window: Window::new(width, height),
        }
    }

    pub fn init(&mut self) {
        self.blackboard.camera = Some(Camera::new(0,0,0));
        self.blackboard.max_action_time = TURN_MAX_TIME;

        self.engine.passive_systems.push(
            Box::new(RenderSystem::new(
                GAME_WIDTH,
                GAME_HEIGHT,
                0, 0
            ))
        );
        self.engine.passive_systems.push(
            Box::new(SelectionRenderSystem::new(
                GAME_WIDTH,
                GAME_HEIGHT
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

        self.engine.load(&mut self.blackboard);

        for i in 0..4 {
            let p = self.engine.component_manager.create_entity();
            self.engine.component_manager
                .set(p, PositionComponent::new(1 + i, 1, 0, 1));
            self.engine.component_manager.set(p, TileComponent::new(
                Tile::new(
                    Some(Color::new_from_rgb(155, 200, 255)),
                    None,
                    160
                )
            ));
            self.engine.component_manager.set(p, ColliderComponent::new(1));

            self.engine.space.add_entity_at(p, Point::new(1 + i, 1, 0));
            self.blackboard.player_entities.insert(p);
            self.scheduler.push_entity(p, 0);
        }

        self.blackboard.current_entity = match self.scheduler.pop_entity() {
            Some((e, dt)) => Some(e),
            None          => None
        };
    }

    pub fn run(&mut self) {
        while self.window.is_open() {
            self.update();
        }
    }

    fn update(&mut self) {
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
                let entity = self.blackboard.current_entity.unwrap();
                next_state = self.keyboard_control(
                    &events,
                    entity
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
                let time_elapsed = delta > 0;
                if time_elapsed {
                    self.engine.update_continuous_systems(
                        &mut self.blackboard,
                        delta
                    );
                }
                if completed {
                    if time_elapsed {
                        self.blackboard.current_action_time += delta;
                    }
                    if self.blackboard.current_action_time
                        >= self.blackboard.max_action_time
                    {
                        self.scheduler.push_entity(
                            self.blackboard.current_entity.unwrap(),
                            self.blackboard.current_action_time
                        );

                        match self.scheduler.pop_entity() {
                            Some((entity, dt)) => {
                                self.blackboard.current_entity =
                                    Some(entity);
                                self.scheduler.elapse_time(dt);
                            }
                            None => {
                                self.blackboard.current_entity = None;
                            }
                        };

                        self.blackboard.current_action_time = 0;
                    }

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
                                    50 //TODO: use dynamic value
                                )
                            )
                        ),
                        (Key::D, false, false, true) => Some(
                            Box::new(
                                MoveAction::new(
                                    entity,
                                    Direction::East,
                                    50
                                )
                            )
                        ),
                        (Key::E, false, false, true) => Some(
                            Box::new(
                                MoveAction::new(
                                    entity,
                                    Direction::Down,
                                    50
                                )
                            )
                        ),
                        (Key::Q, false, false, true)  => Some(
                            Box::new(
                                MoveAction::new(
                                    entity,
                                    Direction::Up,
                                    50
                                )
                            )
                        ),
                        (Key::S, false, false, true) => Some(
                            Box::new(
                                MoveAction::new(
                                    entity,
                                    Direction::South,
                                    50
                                )
                            )
                        ),
                        (Key::W, false, false, true) => Some(
                            Box::new(
                                MoveAction::new(
                                    entity,
                                    Direction::North,
                                    50
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