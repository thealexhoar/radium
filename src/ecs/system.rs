use ecs::{ComponentManager, Space};
use graphics::{GlyphBatch, Window};
use game::Blackboard;

//don't modify any data, just read it
//useful for rendering, animating, etc
pub trait PassiveSystem {
    fn update(
        &mut self,
        blackboard: &mut Blackboard,
        component_manager: &ComponentManager,
        space: &Space,
        glyph_batch: &mut GlyphBatch,
        window: &mut Window,
        delta_time: f32 // real elapsed time
    );

}

//run continuously to handle continuous processes
//ie spawning, day-night cycle
pub trait ContinuousSystem {
    fn update(
        &mut self,
        blackboard: &mut Blackboard,
        component_manager: &mut ComponentManager,
        space: &mut Space,
        delta_time: u32 //game elapsed time
    );
}