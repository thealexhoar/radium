extern crate core;

pub use component::{
    ComponentManager, Component, 
    TurnComponent, PositionComponent};
pub use engine::{Engine};
pub use entity::Entity;
pub use event::{Event, EventType};
use scheduler::Scheduler;
pub use system::{PassiveSystem, ReactiveSystem, ContinuousSystem};
pub use space::Space;

pub mod component;
pub mod engine;
pub mod entity;
pub mod event;
mod scheduler;
pub mod system;
pub mod space;

#[cfg(test)]
mod tests;
