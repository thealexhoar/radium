extern crate core;

pub use component::{Component};
pub use engine::{ComponentManager, Scheduler, Engine};
pub use entity::Entity;
pub use event::{Event, EventType};
pub use system::{PassiveSystem, ReactiveSystem, ContinuousSystem};

pub mod component;
pub mod engine;
pub mod entity;
pub mod event;
pub mod system;

#[cfg(test)]
mod tests;
