use ecs::{
    Component, ComponentManager,
    Entity, Event, EventType, EventResult,
    Family, PositionComponent,
    ReactiveSystem, Space, TurnComponent
};
use game::player::{PlayerAction, PlayerComponent};
use graphics::Event as WindowEvent;
use std::any::TypeId;
use std::ops::Deref;



pub struct PlayerSystem {
    family: Family,
    player: Option<Entity>
}

impl PlayerSystem {
    pub fn new() -> Self {
        Self {
            family: Family::new().all(vec![
                TypeId::of::<PlayerComponent>(),
                TypeId::of::<PositionComponent>(),
                TypeId::of::<TurnComponent>()
            ]),
            player: None
        }
    }

    fn search_for_player(&mut self, component_manager: &mut ComponentManager
    ) -> bool {
        let entities = component_manager.get_entities_for(&self.family);
        match entities.len() {
            0 => false,
            _ => {
                self.player = Some(entities[0]);
                true
            }
        }
    }

    fn take_turn (
        &self,
        component_manager: &mut ComponentManager,
        space: &mut Space
    ) -> EventResult {
        let mut player_component = component_manager
            .get_mut::<PlayerComponent>(self.player.unwrap())
            .unwrap();

        match player_component.next_action {
            Some(player_action) => match player_action {
                _ => EventResult::empty()
            },
            None => EventResult::new(
                Some(vec![
                    Event::new(
                        EventType::TakeTurn,
                        self.player,
                        0,
                        0
                    ),
                    Event::new(
                        EventType::WaitInput,
                        None,
                        0,
                        0
                    )
                ]),
                false
            )
        }
    }

    fn check_if_blocked(
        &self, 
        component_manager: &mut ComponentManager
    ) -> EventResult {
        let mut player_component = component_manager
            .get_mut::<PlayerComponent>(self.player.unwrap())
            .unwrap();
        
        match player_component.next_action {
            Some(action) => EventResult::new(None, false),
            None         => EventResult::empty()
        }
    }

    fn window_input(
        &self,
        component_manager: &mut ComponentManager,
        event: &Event
    ) -> EventResult {
        let mut player_component = component_manager
            .get_mut::<PlayerComponent>(self.player.unwrap())
            .unwrap();
        match player_component.next_action {
            Some(action) => {},
            None         => match event.data
                .deref()
                .downcast_ref::<WindowEvent>() 
                {
                    Some(window_event) => match *window_event {
                        WindowEvent::KeyPress {
                            code, alt, ctrl, shift
                        } => {
                            player_component.next_action = 
                                Self::input_to_action(code, alt, ctrl, shift);
                        },
                        _ => {}
                    },
                    None => {}
                }
        }
        EventResult::empty()
    }

    fn input_to_action(
        code:char,
        alt:bool, ctrl:bool, shift:bool
    ) -> Option<PlayerAction> {
        match (code, alt, ctrl, shift) {
            ('w', false, false, false) => Some(PlayerAction::MoveUp),
            ('s', false, false, false) => Some(PlayerAction::MoveDown),
            ('a', false, false, false) => Some(PlayerAction::MoveLeft),
            ('d', false, false, false) => Some(PlayerAction::MoveRight),
            _                          => None
        }
    }
}

impl ReactiveSystem for PlayerSystem {
    fn update(
        &mut self,
        component_manager: &mut ComponentManager,
        space: &mut Space,
        event: &Event
    ) -> EventResult {
        match self.player {
            Some(entity) => {},
            None         => match self.search_for_player(component_manager) {
                true  => {},
                false => return EventResult::empty() 
            }
        };
        match event.event_type {
            EventType::TakeTurn => self.take_turn(
                component_manager,
                space
            ),
            EventType::WaitInput => self.check_if_blocked(
                component_manager
            ),
            EventType::WindowEvent => self.window_input(
                component_manager,
                event
            ),
            _ => { EventResult::empty() }
        }
    }
}