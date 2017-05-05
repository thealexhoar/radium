use ecs::{Entity, Component};
use std::collections::{HashMap, HashSet};
use std::any::{Any, TypeId};

pub struct ComponentManager {
    //was previously a map of vectors... 
    //hashes by typeid and entity
    _data: HashMap<(TypeId, Entity), Option<Box<Any>>>,
    _entities: HashSet<Entity>,
    _component_types: HashSet<TypeId>,
    _size: usize
}

impl ComponentManager {
    pub fn new() -> ComponentManager {
        ComponentManager {
            _data: HashMap::new(),
            _entities: HashSet::new(),
            _component_types: HashSet::new(),
            _size: 0
        }
    }

    pub fn get<T: Component>(&self, entity:Entity) -> Option<&T> {
        let id = TypeId::of::<T>();
        
        match self._data.get(&(id, entity)) {
            Some(data_option) => match *data_option {
                Some(ref data_box) => data_box.downcast_ref::<T>(),
                //entity component pair is possible, but nonexistant
                None           => None 
            },
            //entity or component doesn't yet exist in manager
            None              => None
        }
    }

    pub fn get_mut<T: Component>(&mut self, entity:Entity) -> Option<&mut T> {
        let id = TypeId::of::<T>();
        match self._data.get_mut(&(id, entity)) {
            Some(data_option) => match *data_option {
                Some(ref mut data_box) => data_box.downcast_mut::<T>(),
                //entity component pair is possible, but nonexistant
                None           => None 
            },
            //entity or component doesn't yet exist in manager
            None              => None
        }
    }

    //fails if entity not yet in manager
    pub fn set<T: Component + Sized>(
        &mut self, 
        entity:Entity, component:T
    ) -> bool { 
        let id = TypeId::of::<T>();
        if !self._entities.contains(&entity) {
            return false;
        }

        if self._component_types.insert(id) {
            for e in self._entities.iter() {
                self._data.insert((id, *e), None);
            }
        }

        self._data.insert((id, entity), Some(Box::new(component)));
        return true;
    }
    
    pub fn add_entity(&mut self, entity: Entity) -> bool {
        if self._entities.insert(entity) {
            for id in self._component_types.iter() {
                self._data.remove(&(*id, entity));
            }
            return true;
        }
        else { return false; }
    }

    pub fn remove_entity(&mut self, entity: Entity) -> bool {
        if self._entities.remove(&entity) {
            for id in self._component_types.iter() {
                self._data.remove(&(*id, entity));
            }
            return true;
        }
        else { return false; }
    }

}