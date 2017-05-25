use ecs::{Component, PositionComponent, Entity, ComponentManager};
use std::collections::{HashMap};
use util::Point;

const CHUNK_SIDE_LEN:usize = 10;

struct Chunk {
    corner: Point,
    data: Vec<Vec<Vec<Entity>>>
}

impl Chunk {
    fn new(left:i32, top:i32) -> Chunk {
        let mut data: Vec<Vec<Vec<Entity>>> = Vec::with_capacity(CHUNK_SIDE_LEN);
        for i in 0..CHUNK_SIDE_LEN {
            let mut next_column: Vec<Vec<Entity>> 
                = Vec::with_capacity(CHUNK_SIDE_LEN);
            for j in 0..CHUNK_SIDE_LEN {
                next_column.insert(j, Vec::new());
            }
            data.insert(i, next_column);
        }
        Chunk {
            corner: Point::new(left, top),
            data
        }
    }

    fn entities_at(
        &self, 
        point: Point
    ) -> &Vec<Entity> {
        let (x,y) = self.local_indices(point);
        &self.data[x][y]
    }

    fn add_entity_at(
        &mut self,
        entity:Entity,
        point: Point
    ) -> bool {
        let (x,y) = self.local_indices(point);
        self.data[x][y].push(entity);
        return true;
    }

    fn remove_entity(
        &mut self,
        entity: Entity,
        point: Point
    ) -> bool {
        let (x,y) = self.local_indices(point);
        let vec = &mut self.data[x][y];
        let mut result = false; 
        for i in 0..vec.len() {
            if vec[i] == entity {
                result = true;
                vec.remove(i);
                break;
            }
        }
        result
    }

    fn local_indices(&self, world_point:Point) -> (usize, usize) {
        let (x,y) = world_point.tuple();
        let corner_x = self.corner.x * CHUNK_SIDE_LEN as i32;
        let corner_y = self.corner.y * CHUNK_SIDE_LEN as i32;
        let transformed_x = x - corner_x;
        let transformed_y = y - corner_y;
        let out_x = (x - corner_x) as usize;
        let out_y = (y - corner_y) as usize;
        (out_x, out_y)
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

        match self._chunks.get(&(chunk_x, chunk_y)) {
            Some(chunk) => Some(chunk.entities_at(point)),
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
