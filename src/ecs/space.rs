use ecs::{Component, PositionComponent, Entity, ComponentManager};
use std::collections::{HashMap};

const CHUNK_SIDE_LEN:i32 = 10;

struct Chunk {
    left: i32,
    top: i32,
    depth: i32,
    data: HashMap<(i32, i32, i32), Vec<Entity>>
}

impl Chunk {
    fn new(left:i32, top:i32, depth:i32) -> Chunk {
        Chunk {
            left,
            top,
            depth,
            data: HashMap::new()
        }
    }

    fn entities_at(
        &self, 
        world_x:i32,
        world_y:i32,
        world_z:i32
    ) -> Option<&Vec<Entity>> {
        match self.data.get(&(world_x, world_y, world_z)) {
            Some(vec) => Some(vec),
            None      => None
        }
    }

    fn add_entity_at(
        &mut self,
        entity:Entity,
        world_x:i32,
        world_y:i32,
        world_z:i32
    ) -> bool {
        if !self.data.contains_key(&(world_x, world_y, world_z)) {
            self.data.insert((world_x, world_y, world_z), Vec::new());
        }
        match self.data.get_mut(&(world_x, world_y, world_z)) {
            Some(ref mut vector) => {
                vector.push(entity);
                true
            },
            None                 => false
        }
    }

    fn remove_entity_at(
        &mut self,
        entity:Entity,
        world_x:i32,
        world_y:i32,
        world_z:i32
    ) -> bool {
        match self.data.get_mut(&(world_x, world_y, world_z)) {
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
    _chunks: HashMap<(i32, i32, i32), Chunk>
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
        world_y:i32,
        world_z:i32
    ) -> Option<&Vec<Entity>> {
        let (chunk_x, chunk_y, chunk_z) = Self::chunk_dimensions(
            world_x, world_y, world_z
        );
        match self._chunks.get(&(chunk_x, chunk_y, chunk_z)) {
            Some(chunk) => chunk.entities_at(world_x, world_y, world_z),
            None        => None
        }
    }

    pub fn add_entity_at(
        &mut self,
        entity:Entity,
        world_x:i32,
        world_y:i32,
        world_z:i32
    ) -> bool {
        let (chunk_x, chunk_y, chunk_z) = Self::chunk_dimensions(
            world_x, world_y, world_z
        );
        match self._chunks.get_mut(&(chunk_x, chunk_y, chunk_z)) {
            Some(ref mut chunk) 
                 => chunk.add_entity_at(entity, world_x, world_y, world_z),
            None => false
        }
    }

    //optimized remove if the entity pos is known
    pub fn remove_entity(
        &mut self,
        entity:Entity,
        world_x:i32,
        world_y:i32,
        world_z: i32
    ) -> bool {
        let (chunk_x, chunk_y, chunk_z) = Self::chunk_dimensions(
            world_x, world_y, world_z
        );
        match self._chunks.get_mut(&(chunk_x, chunk_y, chunk_z)) {
            Some(ref mut chunk) 
                 => chunk.remove_entity_at(entity, world_x, world_y, world_z),
            None => false
        }
    }

    fn chunk_dimensions(x:i32, y:i32, z:i32) -> (i32, i32, i32) {
        (x / CHUNK_SIDE_LEN, y / CHUNK_SIDE_LEN, z / CHUNK_SIDE_LEN)
    }
}
