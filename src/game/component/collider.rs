use ecs::*;

pub struct ColliderComponent {
    pub collision_bits: u32
}

impl Component for ColliderComponent {}

impl ColliderComponent {
    pub fn new(collision_bits: u32) -> Self {
        Self { collision_bits }
    }
}