use std::any::Any;

pub enum EventType {
    Named(String)
}

pub struct Event {
    pub event_type: EventType,
    pub data: Box<Any>,
    pub delta_time: u32
}

impl Event {
    pub fn new<T: Any + Sized>(
        event_type:EventType,
        data:T,
        delta_time:u32
    ) -> Event {
        Event {
            event_type: event_type,
            data: Box::new(data),
            delta_time: delta_time
        }
    }
}