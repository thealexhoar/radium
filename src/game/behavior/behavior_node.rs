use ecs::*;
use game::behavior::BehaviorResult;
use game::Blackboard;

pub trait BehaviorNode {
    fn execute(
        &mut self,
        &mut ComponentManager,
        &mut Space,
        &mut Blackboard
    ) -> BehaviorResult;
}