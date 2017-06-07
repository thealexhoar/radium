use ecs::Entity;
use game::graphics::Camera;

pub struct Blackboard {
    pub camera: Option<Camera>
}

impl Blackboard {
    pub fn new() -> Self {
        Self {
            camera: None
        }
    }
}