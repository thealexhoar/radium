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

    fn remove_entity(
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

    pub fn load_chunk(&mut self, chunk_x: i32, chunk_y:i32 ) {
        //TODO: complex chunk load/unload
        self._chunks.insert((chunk_x, chunk_y), Chunk::new(chunk_x, chunk_y));
    } 

    pub fn entities_at(
        &self, 
        point: Point
    ) -> Option<&Vec<Entity>> {
        let (chunk_x, chunk_y) = Self::chunk_dimensions(
            point.x, point.y
        );

        let b = self._chunks.contains_key(&(chunk_x, chunk_y));

        match self._chunks.get(&(chunk_x, chunk_y)) {
            Some(chunk) => {
                let out = chunk.entities_at(point);
                out
            },
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
                 => chunk.remove_entity(entity, point),
            None => false
        }
    }

    pub fn move_entity(
        &mut self,
        entity: Entity,
        point0: Point,
        point1: Point
    ) -> bool {
        if !self.remove_entity(entity, point0) {
            //don't need to clean up, failure makes no change
            return false;
        }
        if !self.add_entity_at(entity, point1) {
            //do need to clean up the removal
            self.add_entity_at(entity, point0);
            return false;
        }
        return true;
    }

    fn chunk_dimensions(x:i32, y:i32) -> (i32, i32) {
        (x / CHUNK_SIDE_LEN, y / CHUNK_SIDE_LEN)
    }
}
