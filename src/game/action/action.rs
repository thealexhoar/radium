use ecs::{ComponentManager, Space};
use game::Blackboard;


pub trait Action : Sized + Clone {
    //return whether or not it is completed, and game time elapsed at each step
    fn execute(
        &mut self,
        component_manager: &mut ComponentManager,
        space: &mut Space,
        blackboard: &mut Blackboard,
        delta_time: f32
    ) -> (bool, bool, u32);
}