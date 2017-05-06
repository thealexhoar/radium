use std::any::Any;
use ecs::Entity;

#[derive(Copy, Clone)]
pub enum EventType {
    Initialize,
    WindowEvent,
    Custom(u32)
}

pub struct Event {
    pub event_type: EventType,
    pub entity_source: Option<Entity>,
    pub data: Box<Any>,
    pub delta_time: u32
}

impl Event {
    pub fn new<T: Any + Sized>(
        event_type:EventType,
        entity_source: Option<Entity>,
        data:T,
        delta_time:u32
    ) -> Event {
        Event {
            event_type,
            entity_source,
            data: Box::new(data),
            delta_time
        }
    }
}