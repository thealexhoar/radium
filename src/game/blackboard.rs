use ecs::Entity;
use game::graphics::Camera;

pub struct Blackboard {
    pub player: Option<Entity>,
    pub camera: Option<Camera>
}

impl Blackboard {
    pub fn new() -> Self {
        Self {
            player: None,
            camera: None
        }
    }
}