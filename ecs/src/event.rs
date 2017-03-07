pub enum EventType {
    Named(String)
}

pub trait Event : 'static {
    //TODO: add a serialize function
}