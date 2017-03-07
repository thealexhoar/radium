use PositionComponent;
use Component;
use ComponentManager;
use Entity;
use std::collections::{HashMap};

const CHUNK_SIDE_LEN:i32 = 10;
struct Chunk {
    top: i32,
    left: i32,
    data: HashMap<(i32, i32), Vec<Entity>>
}

impl Chunk {
    fn new(top:i32, left:i32) -> Chunk {
        Chunk {
            top: top,
            left: left,
            data: HashMap::new()
        }
    }

    fn entities_at(
        &self, 
        world_x:i32,
        world_y:i32
    ) -> Option<&Vec<Entity>> {
        match self.data.get(&(world_x, world_y)) {
            Some(vec) => Some(vec),
            None      => None
        }
    }

    fn remove_entity(&mut self, entity:Entity) -> bool {
        'outer: for (pos, ref mut vector) in &self.data{
            for i in 0..vector.len() {
                if vector[i] == entity {
                    return true;
                }
            }
        }
        false
    }

    fn remove_entity_at(
        &mut self,
        entity:Entity,
        world_x:i32,
        world_y:i32
    ) -> bool {
        match self.data.get_mut(&(world_x, world_y)) {
            Some(ref mut vector) => {
                let mut result = false;
                for i in 0..vector.len() {
                    if vector[i] == entity {
                        result = true;
                        break;
                    }
                }
                result
            },
            None                 => false
        }
    }
}

pub struct Space {
    _chunks: HashMap<(i32, i32), Chunk>
}

impl Space {
    pub fn new() -> Space {
        Space {
            _chunks: HashMap::new()
        }
    }

    pub fn entities_at(
        &self, 
        world_x:i32,
        world_y:i32
    ) -> Option<&Vec<Entity>> {
        let chunk_x = world_x / CHUNK_SIDE_LEN;
        let chunk_y = world_y / CHUNK_SIDE_LEN;
        match self._chunks.get(&(chunk_x, chunk_y)) {
            Some(chunk) => chunk.entities_at(world_x, world_y),
            None        => None
        }
    }

    pub fn remove_entity(&mut self, entity:Entity) -> bool {
        let mut pos:Option<(i32, i32)> = None;
        let mut result = false;
        for (pos, mut chunk) in &mut self._chunks {
            if chunk.remove_entity(entity) {
                result = true;
                break;
            }
        }
        result
    }

    pub fn remove_entity_at(
        &mut self,
        entity:Entity,
        world_x:i32,
        world_y:i32
    ) -> bool {
        let chunk_x = world_x / CHUNK_SIDE_LEN;
        let chunk_y = world_y / CHUNK_SIDE_LEN;
        match self._chunks.get_mut(&(chunk_x, chunk_y)) {
            Some(ref mut chunk) 
                 => chunk.remove_entity_at(entity, world_x, world_y),
            None => false
        }
    }
}
