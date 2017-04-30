use ecs::{
    PassiveSystem, ReactiveSystem, ContinuousSystem,
    Component, ComponentManager,
    Entity,
    EventType, Event,
    Scheduler 
};
use util::PriorityQueue;


pub struct Engine {
    _passive_systems: PriorityQueue<Box<PassiveSystem>>,
    _reactive_systems: PriorityQueue<Box<ReactiveSystem>>,
    _continuous_systems: PriorityQueue<Box<ContinuousSystem>>,
    _component_manager: ComponentManager,
    _scheduler: Scheduler
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            _passive_systems: PriorityQueue::new(),
            _reactive_systems: PriorityQueue::new(),
            _continuous_systems: PriorityQueue::new(),
            _component_manager: ComponentManager::new(),
            _scheduler: Scheduler::new()
        }
    }

    pub fn add_passive_system<T: 'static + PassiveSystem + Sized>(
        &mut self,
        system:T,
        priority:i32
    ) {
        let system_box = Box::new(system);
        self._passive_systems.insert(system_box, priority);
    }

    pub fn add_reactive_system<T: 'static + ReactiveSystem + Sized> (
        &mut self,
        system:T,
        priority:i32
    ) {
        let system_box = Box::new(system);
        self._reactive_systems.insert(system_box, priority);
    }

    pub fn add_continuous_system<T: 'static + ContinuousSystem + Sized>(
        &mut self,
        system:T,
        priority:i32
    ) {
        let system_box = Box::new(system);
        self._continuous_systems.insert(system_box, priority);
    }

    pub fn update(&mut self, delta_time:f32) {
        //update calls go here
    }
}
