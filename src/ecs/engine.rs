use {PassiveSystem, ReactiveSystem, ContinuousSystem};
use {Component, ComponentManager};
use Entity;
use {EventType, Event};
use Scheduler;


pub struct Engine {
    _passive_systems: Vec<Box<PassiveSystem>>,
    _reactive_systems: Vec<Box<ReactiveSystem>>,
    _continuous_systems: Vec<Box<ContinuousSystem>>,
    _component_manager: ComponentManager,
    _scheduler: Scheduler
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            _passive_systems: Vec::new(),
            _reactive_systems: Vec::new(),
            _continuous_systems: Vec::new(),
            _component_manager: ComponentManager::new(),
            _scheduler: Scheduler::new()
        }
    }
}
