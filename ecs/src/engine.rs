use {PassiveSystem, ReactiveSystem, ContinuousSystem};
use {Component};
use Entity;
use std::collections::{HashMap};
use std::collections::hash_map::{Entry, OccupiedEntry};
use std::any::{Any, TypeId};
use std::ops::{Deref, DerefMut};

pub struct ComponentManager {
    _data: HashMap<TypeId, Vec<Option<Box<Any>>>>//wewlad
}

impl ComponentManager {
    pub fn new() -> ComponentManager {
        ComponentManager {
            _data: HashMap::new()
        }
    }

    pub fn get(&self, entity:Entity, id:&TypeId) -> Option<&Any> {
        match self._data.get(id) { 
            Some(data_vec) => {
                match data_vec[entity as usize] {
                    Some(ref data) =>  Some(data.deref()),
                    None       => None
                }
            },
            None       => None
        }
    }
    
    pub fn get_mut(&mut self, entity:Entity, id:&TypeId) -> Option<&mut Any> {
        match self._data.get_mut(&id) { 
            Some(data_vec) => {
                match data_vec[entity as usize] {
                    Some(ref mut data) => Some(data.deref_mut()),
                    None       => None
                }
            },
            None       => None
        }
    }
}

pub struct Scheduler {
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
            _scheduler: Scheduler{}
        }
    }
}
