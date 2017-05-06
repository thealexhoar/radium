use ecs::{ComponentManager, Space, Entity, Event};
use behavior::Result;

pub trait BehaviorTreeNode {
    fn execute(
        &mut self,
        entity: Entity,
        component_manager: &mut ComponentManager,
        space: &mut Space
    ) -> (Result, Option<Vec<Event>>);
}