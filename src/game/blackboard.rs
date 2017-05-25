use ecs::Entity;

pub struct Blackboard {
    pub player: Option<Entity>
}

impl Blackboard {
    pub fn new() -> Self {
        Self {
            player: None
        }
    }
}