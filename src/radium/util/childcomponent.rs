use ecs::{Component, Entity};

pub struct ChildComponent {
    pub children: Vec<Entity>
}

impl Component for ChildComponent {}

impl ChildComponent {
    pub fn new() -> Self {
        Self {
            children: Vec::new()
        }
    }
}