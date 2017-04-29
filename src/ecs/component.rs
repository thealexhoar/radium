use Entity;
use std::collections::{HashMap};
use std::any::{Any, TypeId};

pub trait Component : 'static{
    //TODO: add a serialize function
}

//core components
//TODO: find a better home for these little guys?

pub struct PositionComponent {
    pub x:i32,
    pub y:i32
}
impl Component for PositionComponent {}

pub struct TurnComponent;
impl Component for TurnComponent {}


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
                None               => None
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