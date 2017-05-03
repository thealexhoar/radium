use ecs::{Component, PositionComponent, Entity, ComponentManager};
use std::collections::{HashMap};
use util::Point;

const CHUNK_SIDE_LEN:i32 = 10;

struct Chunk {
    corner: Point,
    data: HashMap<(i32, i32), Vec<Entity>>
}

impl Chunk {
    fn new(left:i32, top:i32) -> Chunk {
        Chunk {
            corner: Point::new(left, top),
            data: HashMap::new()
        }
    }

    fn entities_at(
        &self, 
        point: Point
    ) -> Option<&Vec<Entity>> {
        match self.data.get(&point.tuple()) {
            Some(vec) => Some(vec),
            None      => None
        }
    }

    fn add_entity_at(
        &mut self,
        entity:Entity,
        point: Point
    ) -> bool {
        if !self.data.contains_key(&point.tuple()) {
            self.data.insert(point.tuple(), Vec::new());
        }
        match self.data.get_mut(&point.tuple()) {
            Some(ref mut vector) => {
                vector.push(entity);
                true
            },
            None                 => false
        }
    }

    fn remove_entity_at(
        &mut self,
        entity: Entity,
        point: Point
    ) -> bool {
        match self.data.get_mut(&point.tuple()) {
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
        point: Point
    ) -> Option<&Vec<Entity>> {
        let (chunk_x, chunk_y) = Self::chunk_dimensions(
            point.x, point.y
        );
        match self._chunks.get(&(chunk_x, chunk_y)) {
            Some(chunk) => chunk.entities_at(point),
            None        => None
        }
    }

    pub fn add_entity_at(
        &mut self,
        entity: Entity,
        point: Point 
    ) -> bool {
        let (chunk_x, chunk_y) = Self::chunk_dimensions(
            point.x, point.y
        );
        match self._chunks.get_mut(&(chunk_x, chunk_y)) {
            Some(ref mut chunk) 
                 => chunk.add_entity_at(entity, point),
            None => false
        }
    }

    //optimized remove if the entity pos is known
    pub fn remove_entity(
        &mut self,
        entity:Entity,
        point: Point
    ) -> bool {
        let (chunk_x, chunk_y) = Self::chunk_dimensions(
            point.x, point.y
        );
        match self._chunks.get_mut(&(chunk_x, chunk_y,)) {
            Some(ref mut chunk) 
                 => chunk.remove_entity_at(entity, point),
            None => false
        }
    }

    fn chunk_dimensions(x:i32, y:i32) -> (i32, i32) {
        (x / CHUNK_SIDE_LEN, y / CHUNK_SIDE_LEN)
    }
}
