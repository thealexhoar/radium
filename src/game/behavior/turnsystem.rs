use ecs::{
    Component, ComponentManager,
    Event, EventResult, EventType, 
    ReactiveSystem, Space
};
use behavior::Behavior;
use game::behavior::BehaviorComponent;
use std::ops::DerefMut;

pub struct TurnTakerSystem {

}

impl ReactiveSystem for TurnTakerSystem {
    fn update(
        &mut self,
        component_manager: &mut ComponentManager,
        space: &mut Space,
        event: &Event
    ) -> EventResult {
        match event.event_type {
            EventType::TakeTurn => {
                let entity = match event.entity_source {
                    Some(e) => e,
                    None    => return EventResult::empty()
                };
                //remove from component manager
                let mut bc_box = match component_manager
                    .remove_box::<BehaviorComponent>(entity) {
                    Some(component_box) => component_box,
                    None            => return EventResult::empty()
                };
                //TODO: implement

                let (result, out_events) = 
                    bc_box.deref_mut().behavior_box.deref_mut().execute(
                    entity,
                    component_manager,
                    space
                );

                //return to component manager
                component_manager.set_box(entity, bc_box);
                return EventResult::new(out_events, true);

            },
            _                   => return EventResult::empty()
        };
    }
}