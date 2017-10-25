use ecs::{ComponentManager, Space};
use game::Blackboard;
use game::action::Action;

pub struct WaitAction {
}

impl WaitAction {
    pub fn new() -> Self {
        Self {}
    }
}

impl Action for WaitAction {
    fn execute(
        &mut self,
        component_manager: &mut ComponentManager,
        space: &mut Space,
        blackboard: &mut Blackboard,
        delta_time: f32
    ) -> (bool, bool, u32) {
        (true, true, 0)
    }
}