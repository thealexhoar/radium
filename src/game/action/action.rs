use ecs::*;

pub trait Action {
    //return whether or not it is completed, and game time elapsed at each step
    fn execute(
        &mut self,
        component_manager: &mut ComponentManager,
        space: &mut Space,
        delta_time: f32
    ) -> (bool, u32);
}