use game::action::actiontypes::*;
use ecs::*;
use util::*;
use std::ops::Deref;

pub fn construct_move_event(
    entity:Entity, 
    delta:Point, 
    delta_time:u32
) -> Event {
    Event::new(
        EventType::Action(MOVE),
        Some(entity),
        delta,
        delta_time
    )
}

pub struct MoveSystem {
    
}

impl MoveSystem {
    pub fn new() -> Self {
        Self {}
    }
}

impl ReactiveSystem for MoveSystem {
    fn update(
        &mut self,
        component_manager: &mut ComponentManager,
        space: &mut Space,
        event: &Event
    ) -> EventResult {
        match event.event_type {
            EventType::Action(action) => {
                if action == MOVE {
                    //TODO: collision checking?
                    let entity = event.entity_source.unwrap();
                    let pc = component_manager
                        .get_mut::<PositionComponent>(entity).unwrap();
                    
                    let delta = event.data.deref()
                        .downcast_ref::<Point>().unwrap();

                    space.remove_entity(entity, pc.point);

                    pc.point = pc.point + *delta;
                    
                    space.add_entity_at(entity, pc.point);
                }
                
            },
            _ => {}
        };
        
        EventResult::empty()
    }
}