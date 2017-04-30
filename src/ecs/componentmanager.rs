use ecs::{Entity, Component};
use std::collections::{HashMap};
use std::any::{Any, TypeId};

pub struct ComponentManager {
    _data: HashMap<TypeId, Vec<Option<Box<Any>>>>,
    _entities: Vec<Option<Entity>>,
    _entity_indices: HashMap<Entity, usize>,
    _size: usize
}

//TODO: improve entity storage data structure
impl ComponentManager {
    pub fn new() -> ComponentManager {
        ComponentManager {
            _data: HashMap::new(),
            _entities: Vec::new(),
            _entity_indices: HashMap::new(),
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
    
    pub fn add_entity(&mut self, entity: Entity) {
        let mut final_index = self._entities.len();
        for (index, entity_val) in self._entities.iter().enumerate() {
            match entity_val {
                &Some(e) => {},
                &None    => {
                    final_index = index;
                    break;
                }
            }
        }
        if final_index >= self._entities.capacity() {
            self._entities.resize(final_index + 32, None);
        }
        self._entities[final_index] = Some(entity);
        self._entity_indices.insert(entity, final_index);
    }

    pub fn remove_entity(&mut self, entity: Entity) {
        let index = match self._entity_indices.get(&entity) {
            Some(i) => *i,
            None    => return
        };
        self._entities[index] = None;
        for component_vec in self._data.values_mut() {
            component_vec[index] = None;
        }
        self._entity_indices.remove(&entity);
    }

    pub fn get_entity_index(&mut self, entity: Entity) -> Option<usize> {
        match self._entity_indices.get(&entity) {
            Some(i) => Some(*i),
            None    => None
        }
    }
}