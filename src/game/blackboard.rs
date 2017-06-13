use ecs::Entity;
use game::graphics::Camera;
use std::collections::HashSet;

pub struct Blackboard {
    pub camera: Option<Camera>,
    pub current_entity: Option<Entity>,
    pub player_entities: HashSet<Entity>,
    pub enemy_entities: HashSet<Entity>
}

impl Blackboard {
    pub fn new() -> Self {
        Self {
            camera: None,
            current_entity: None,
            player_entities: HashSet::new(),
            enemy_entities: HashSet::new(),
        }
    }
}