use Data;

pub enum EventType {
    Named(String)
}

pub struct Event {
    pub event_type: EventType,
    pub data: Data
}