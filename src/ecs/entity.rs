pub type Entity = u32;

pub struct EntityFactory {
    _entity_count: u32
}

impl EntityFactory {
    pub fn new() -> EntityFactory {
        EntityFactory { _entity_count: 0 }
    }

    pub fn create_entity(&mut self) -> Entity {
        self._entity_count += 1;
        self._entity_count
    }
}
