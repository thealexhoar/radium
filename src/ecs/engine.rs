use ecs::{
    PassiveSystem, ReactiveSystem, ContinuousSystem,
    Component, ComponentManager,
    Entity,
    EventType, Event,
    Scheduler,
    Space,
    TestReactor
};
use util::PriorityQueue;


pub struct Engine {
    _passive_systems: PriorityQueue<Box<PassiveSystem>>,
    _reactive_systems: PriorityQueue<Box<ReactiveSystem>>,
    _continuous_systems: PriorityQueue<Box<ContinuousSystem>>,
    _component_manager: ComponentManager,
    _scheduler: Scheduler,
    _space: Space
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            _passive_systems: PriorityQueue::new(),
            _reactive_systems: PriorityQueue::new(),
            _continuous_systems: PriorityQueue::new(),
            _component_manager: ComponentManager::new(),
            _scheduler: Scheduler::new(),
            _space: Space::new()
        }
    }

    pub fn load(&mut self) {
        //initializes startup
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

    pub fn update(&mut self, true_delta_time:f32) {
        'outer: loop {
            let next_event = match self._scheduler.pop_event() {
                Some(event) => event,
                None        => break 'outer
            };
            let mut event_accepted = true;
            
            'inner: for i in 0..self._reactive_systems.len() {
                let results = self._reactive_systems[i].update(
                    &mut self._component_manager,
                    &mut self._space,   
                    &next_event
                );
                match results.resulting_events {
                    Some(result_events) => for response_event in result_events {
                        self._scheduler.push_event(response_event);
                    },
                    None               => {}
                }
                if !results.allow_event {
                    event_accepted = false;
                    break 'inner;
                }
            }

            if event_accepted {
                for i in 0..self._continuous_systems.len() {
                    self._continuous_systems[i].update(
                        &mut self._component_manager,
                        &mut self._space,
                        next_event.delta_time
                    );
                }
                for i in 0..self._passive_systems.len() {
                    self._passive_systems[i].update(
                        & self._component_manager,
                        & self._space,
                        true_delta_time
                    );
                }
            }

            if next_event.delta_time != 0 {
                break;
            }
        }
    }
}
