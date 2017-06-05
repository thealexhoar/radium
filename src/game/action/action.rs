use ecs::*;

pub trait Action {
    fn execute(
        &mut self,
        component_manager: &mut ComponentManager,
        space: &mut Space,
        delta_time: f32
    ) -> (bool, u32);
    
    fn reset(&mut self);
}