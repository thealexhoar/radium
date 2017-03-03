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

    pub fn get<T: Component>(&self, entity:Entity) -> Option<&T> {
        let id = TypeId::of::<T>();
        match self._data.get(&id) { 
            Some(data_vec) =>  match data_vec[entity as usize] {
                Some(ref data_box) => data_box.downcast_ref::<T>(),
                None           => None
            },
            None       => None
        }
    }

    pub fn get_mut<T: Component>(&mut self, entity:Entity) -> Option<&mut T> {
        let id = TypeId::of::<T>();
        match self._data.get_mut(&id) { 
            Some(data_vec) => match data_vec[entity as usize] {
                Some(ref mut data_box) => data_box.downcast_mut::<T>(),
                None           => None
            },
            None       => None
        }
    }

    pub fn set<T: Component + Any>(&mut self, entity:Entity, component:T) {
        let id = TypeId::of::<T>();
        match self._data.get_mut(&id) { 
            Some(data_vec) => 
                data_vec[entity as usize] = Some(Box::new(component)),
            None           => {
                let mut new_vec = Vec::new();
                new_vec.reserve(entity as usize + 1);
                new_vec[entity as usize] = Some(Box::new(component) as Box<Any>);
                self._data.insert(id, new_vec);
            }
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
