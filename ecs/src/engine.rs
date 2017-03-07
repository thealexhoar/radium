use {PassiveSystem, ReactiveSystem, ContinuousSystem};
use {Component};
use Entity;
use {EventType, Event};
use std::collections::{HashMap};
use std::any::{Any, TypeId};

pub struct ComponentManager {
    _data: HashMap<TypeId, Vec<Option<Box<Any>>>>,//wewlad
    _size: usize
}

impl ComponentManager {
    pub fn new() -> ComponentManager {
        ComponentManager {
            _data: HashMap::new(),
            _size: 0
        }
    }

    pub fn get<T: Component>(&self, entity:Entity) -> Option<&T> {
        let id = TypeId::of::<T>();
        match self._data.get(&id) { 
            Some(data_vec) =>  match data_vec[entity as usize] {
                Some(ref data_box) => data_box.downcast_ref::<T>(),
                None           => None
            },
            None           => None
        }
    }

    pub fn get_mut<T: Component>(&mut self, entity:Entity) -> Option<&mut T> {
        let id = TypeId::of::<T>();
        match self._data.get_mut(&id) { 
            Some(data_vec) => match data_vec[entity as usize] {
                Some(ref mut data_box) => data_box.downcast_mut::<T>(),
                None           => None
            },
            None           => None
        }
    }

    pub fn set<T: Component + Sized>(&mut self, entity:Entity, component:T) { 
        let id = TypeId::of::<T>();
        if !(self._data.contains_key(&id)) {
            self._data.insert(id, Vec::new());
        }
        match self._data.get_mut(&id) {
            Some(ref mut data_vec) => 
                data_vec[entity as usize] 
                = Some(Box::new(component) as Box<Any>),
            None                   => {/* should never be reached */}
        };
    }

    fn get_entity_index(&mut self, entity:Entity) -> usize {
        0
    }
}


struct ScheduledEvent {
    pub event_type: EventType,
    pub event: Box<Event>,
    pub delta_time: u32
}


pub struct Scheduler {
    _event_queue: Vec<ScheduledEvent>,
    _turn_taker_queue: Vec<Entity>
}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            _event_queue: Vec::new(),
            _turn_taker_queue: Vec::new()
        }
    }

    pub fn schedule_event<T: Event + Sized>(
        &mut self, 
        event_type: EventType, 
        event: T,
        delta_time: u32
    ) {
        let scheduled_event = ScheduledEvent{
            event_type: event_type,
            event: Box::new(event),
            delta_time: delta_time
        };
        let mut index:Option<usize> = None;
        for i in 0..self._event_queue.len() {
            if self._event_queue[i].delta_time > delta_time {
                index = Some(i);
                break;
            }
        }
        match index {
            Some(i) => self._event_queue.insert(i, scheduled_event),
            None    => self._event_queue.push(scheduled_event)
        };
    }

    pub fn pop(&mut self) -> Option<(EventType, Box<Event>)> {
        None
    }

    fn subtract_time(&mut self, time:u32) {
        for scheduled_event in &mut self._event_queue {
            scheduled_event.delta_time -= time;
        }
    }
}

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
