pub use ecs::*;
pub use graphics::*;

pub trait Controller {
    //returns delta for controlled entity's reschedule
    fn update(
        &mut self,
        component_manager: &mut ComponentManager,
        space: &mut Space,
        window: &mut Window,
        entity: Entity
    ) -> u32;
}