use std::any::Any;
use std::ops::Deref;
use ecs::{Entity, Event, EventType};

pub trait EventBuilder {
    fn build_event(&self) -> Event;
}

pub struct PrototypeEventBuilder<T: Any + Clone + Sized> {
    pub event_type: EventType,
    pub entity_source: Option<Entity>,
    pub data: Box<T>,
    pub delta_time: u32
}

impl<T: Any + Clone + Sized> PrototypeEventBuilder<T> {
    pub fn new(
        event_type:EventType,
        entity_source: Option<Entity>,
        data:T,
        delta_time:u32
    ) -> Self {
        Self {
            event_type,
            entity_source,
            data: Box::new(data),
            delta_time
        }
    }
}

impl<T: Any + Clone + Sized> EventBuilder for PrototypeEventBuilder<T> {
    fn build_event(&self) -> Event {
        Event::new(
            self.event_type,
            self.entity_source,
            self.data.deref().clone(),
            self.delta_time
        )
    }
}