use ecs::Entity;
use game::graphics::Camera;
use std::collections::HashSet;

pub struct Blackboard {
    pub camera: Option<Camera>,
    pub current_entity: Option<Entity>,
    pub current_action_time:u32,
    pub max_action_time:u32,
    pub player_entities: HashSet<Entity>,
    pub enemy_entities: HashSet<Entity>
}

impl Blackboard {
    pub fn new() -> Self {
        Self {
            camera: None,
            current_entity: None,
            current_action_time: 0,
            max_action_time: 0,
            player_entities: HashSet::new(),
            enemy_entities: HashSet::new(),
        }
    }
}