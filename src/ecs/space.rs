use ecs::{Entity};
use std::collections::{HashMap};
use util::Point;

const CHUNK_SIDE_LEN:usize = 10;

struct Chunk {
    pub west: i32,
    pub north: i32,
    pub data:  HashMap<(i32, i32, u32), Vec<Entity>>
}

impl Chunk {
    fn new(west:i32, north:i32) -> Chunk {
        Chunk {
            west,
            north,
            data: HashMap::new()
        }
    }

    fn entities_at(
        &self,
        point: Point
    ) -> Option<&Vec<Entity>> {
        self.data.get(&point.tuple())
    }

    fn add_entity_at(
        &mut self,
        entity:Entity,
        point: Point
    ) -> bool {
        if !self.data.contains_key(&point.tuple()) {
                self.data.insert(point.tuple(), Vec::new());
        }
        self.data.get_mut(&point.tuple()).unwrap().push(entity);
        return true;
    }

    fn remove_entity(
        &mut self,
        entity: Entity,
        point: Point
    ) -> bool {
        match self.data.get_mut(&point.tuple()) {
            Some(data_vec) => {
                for i in 0..data_vec.len() {
                    if data_vec[i] == entity {
                        data_vec.remove(i);
                        return true;
                    }
                }
            },
            None           => {}
        };
        return false;
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

    pub fn load_chunk(&mut self, chunk_x: i32, chunk_y:i32) {
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
        let mut chunk_x;
        let mut chunk_y;
        if x < 0 {
            chunk_x = (x - 9) / (CHUNK_SIDE_LEN as i32);
        }
        else {
            chunk_x = x / (CHUNK_SIDE_LEN as i32);
        }
        if y < 0 {
            chunk_y = (y - 9) / (CHUNK_SIDE_LEN as i32);
        }
        else {
            chunk_y = y / (CHUNK_SIDE_LEN as i32);
        }

        (chunk_x, chunk_y)
    }
}
