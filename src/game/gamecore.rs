use ecs::*;
use graphics::*;
use game::action::*;
use game::components::ColliderComponent;
use game::Blackboard;
use game::Direction;
use game::render::*;
use game::ui::*;
use std::cmp::max;
use std::ops::DerefMut;
use util::Point;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CoreState {
    PlayerTurn,
    EnemyTurn,

    PlayerAction,
    EnemyAction
}

pub struct GameCore {
    width: u32,
    height: u32,
    current_action: Option<Box<Action>>,
    state: CoreState
}


impl GameCore {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            current_action: None,
            state: CoreState::PlayerTurn
        }
    }

    pub fn update(
        &mut self,
        blackboard: &mut Blackboard,
        engine: &mut Engine,
        glyphbatch: &mut GlyphBatch,
        mouse_interface: &mut MouseInterface,
        scheduler: &mut Scheduler,
        window: &mut Window
    ) {
        let delta_time = window.elapsed_time();

        mouse_interface.update(window);

        engine.update_passive_systems(
            glyphbatch,
            window,
            blackboard,
            delta_time
        );

        window.clear();
        glyphbatch.flush_tiles();
        window.draw_glyphbatch(&glyphbatch);

        let events = window.events();

        

        let mut next_state = self.state;
        match self.state {
            CoreState::PlayerTurn         => {
                //listen for actions pertaining to selected unit
                let entity = blackboard.current_entity.unwrap();
                next_state = self.keyboard_control(
                    &events,
                    entity
                );
                if self.state == next_state {
                    next_state = self.mouse_control(&events);
                }

                self.control_camera(
                    blackboard,
                    &events,
                );
            },

            CoreState::EnemyTurn          => {
                //iterate through enemy controllers
            },

            CoreState::PlayerAction |
            CoreState::EnemyAction        => {
                let (completed, end_turn, delta) = match self.current_action {
                    Some(ref mut action_box) => action_box
                        .deref_mut()
                        .execute(
                            &mut engine.component_manager,
                            &mut engine.space,
                            blackboard,
                            delta_time
                    ),
                    None => (true, false, 0)
                };
                let time_elapsed = delta > 0;
                if time_elapsed {
                    engine.update_continuous_systems(
                        blackboard,
                        delta
                    );
                }
                
                if completed {
                    if time_elapsed {
                        blackboard.current_action_time += delta;
                    }
                    if blackboard.current_action_time
                        >= blackboard.max_action_time
                        || end_turn
                    {
                        if end_turn {
                            blackboard.current_action_time =
                                max(150, blackboard.current_action_time);
                            //TODO: make dynamic
                        }
                        scheduler.push_entity(
                            blackboard.current_entity.unwrap(),
                            blackboard.current_action_time
                        );

                        match scheduler.pop_entity() {
                            Some((entity, dt)) => {
                                blackboard.current_entity =
                                    Some(entity);
                                scheduler.elapse_time(dt);
                            }
                            None => {
                                blackboard.current_entity = None;
                            }
                        };

                        blackboard.current_action_time = 0;
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

        window.update_event_queue();
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
                        (Key::Space, false, false, true) => Some(
                            Box::new(WaitAction::new())
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
        blackboard: &mut Blackboard,
        events: &Vec<Event>
    ) {
        let camera = match blackboard.camera {
            Some(ref mut c) => c,
            None            => { return; }
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