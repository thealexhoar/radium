use ecs::{ComponentManager, Event, Space};

pub struct EventResult{
    pub resulting_events: Vec<Event>,
    pub allow_event: bool
}

//don't modify any data, just read it
//useful for rendering, animating, etc
pub trait PassiveSystem {
    fn update(
        &mut self,
        component_manager: &ComponentManager,
        space: &Space,
        delta_time: f32 // real elapsed time
    ) -> EventResult;
}

//respond to actions
pub trait ReactiveSystem {
    fn update(
        &mut self,
        component_manager: &mut ComponentManager,
        space: &mut Space,
        event: &Event
    ) -> EventResult;
}

//run continuously to handle continuous processes
//ie spawning, day-night cycle
pub trait ContinuousSystem {
    fn update(
        &mut self,
        component_manager: &mut ComponentManager,
        space: &mut Space,
        delta_time: u32 //game elapsed time
    ) -> EventResult;
}