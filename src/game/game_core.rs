use core_manager::CoreState;
use ecs::{ComponentManager, Engine, Entity, Scheduler, Space};
use graphics::{Event, GlyphBatch, Key, Window};
use game::action::{Action, MoveAction, WaitAction};
use game::Blackboard;
use game::Direction;
use game::ui::MouseInterface;
use std::cmp::max;
use std::ops::DerefMut;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    PlayerTurn,
    EnemyTurn,

    PlayerAction,
    EnemyAction
}

pub struct GameCore {
    current_action: Option<Box<Action>>,
    state: GameState
}


impl GameCore {
    pub fn new() -> Self {
        Self {
            current_action: None,
            state: GameState::PlayerTurn
        }
    }

    pub fn init(
        &mut self,
        blackboard: &mut Blackboard,
        engine: &mut Engine,
        glyph_batch: &mut GlyphBatch,
        mouse_interface: &mut MouseInterface,
        scheduler: &mut Scheduler,
        window: &mut Window
    ) {

    }

    pub fn update(
        &mut self,
        blackboard: &mut Blackboard,
        engine: &mut Engine,
        glyph_batch: &mut GlyphBatch,
        mouse_interface: &mut MouseInterface,
        scheduler: &mut Scheduler,
        window: &mut Window
    ) -> CoreState {
        let mut out_state = CoreState::Game;

        let delta_time = window.elapsed_time();

        mouse_interface.update(window);

        engine.update_passive_systems(
            glyph_batch,
            window,
            blackboard,
            delta_time
        );


        window.clear();
        glyph_batch.flush_tiles();
        window.draw_glyph_batch(&glyph_batch);

        let events = window.events();

        let mut next_state = self.state;
        match self.state {
            GameState::PlayerTurn         => {
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

            GameState::EnemyTurn          => {
                //iterate through enemy controllers
            },

            GameState::PlayerAction |
            GameState::EnemyAction        => {
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
                                max(blackboard.max_action_time, blackboard.current_action_time);
                            //TODO: make dynamic?
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
                        GameState::PlayerAction =>
                            next_state = GameState::PlayerTurn,
                        GameState::EnemyAction    =>
                            next_state = GameState::EnemyTurn,
                        _ => {}
                    }
                }
            },

            _ => {}
        };

        self.state = next_state;

        window.update_event_queue();

        out_state
    }

    fn keyboard_control(
        &mut self,
        events: &Vec<Event>,
        entity: Entity
    ) -> GameState {
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
                        return GameState::PlayerAction
                    }
                },
                _ => {}
            }
        }
        GameState::PlayerTurn
    }

    fn mouse_control(
        &mut self,
        events: &Vec<Event>
    ) -> GameState {
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