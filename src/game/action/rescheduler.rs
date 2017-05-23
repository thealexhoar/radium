use ecs::*;

pub struct Rescheduler {
    
}

impl Rescheduler {
    pub fn new() -> Self {
        Self {}
    }
}

impl ReactiveSystem for Rescheduler {
    fn update(
        &mut self,
        component_manager: &mut ComponentManager,
        space: &mut Space,
        event: &Event
    ) -> EventResult {
        match event.event_type {
            EventType::Action(action) => {
                let entity = event.entity_source.unwrap();
                EventResult::new(
                    Some(vec![
                        Event::new(
                            EventType::TakeTurn,
                            Some(entity),
                            0,
                            1
                        )
                    ]),
                    true
                )
            },
            _ => EventResult::empty()
        }
    }
}