use ecs::{ComponentManager, Entity, Space};
use graphics::{Window};
use game::Blackboard;

pub trait Controller {
    //returns delta for controlled entity's reschedule
    fn update(
        &mut self,
        blackboard: &mut Blackboard,
        component_manager: &mut ComponentManager,
        space: &mut Space,
        window: &mut Window,
        entity: Entity
    ) -> (u32, bool);
}