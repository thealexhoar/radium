pub use component::{Component, ComponentType};
pub use data::Data;
pub use engine::{ComponentManager, Engine};
pub use event::{Event, EventType};
pub use system::{PassiveSystem, ReactiveSystem, ContinuousSystem};

pub mod component;
pub mod data;
pub mod engine;
pub mod event;
pub mod system;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
