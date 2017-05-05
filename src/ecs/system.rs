use ecs::{ComponentManager, Event, Space};
use graphics::Window;

pub struct EventResult{
    pub resulting_events: Option<Vec<Event>>,
    pub allow_event: bool
}

impl EventResult {
    pub fn empty(allow_event:bool) -> EventResult {
        EventResult {
            resulting_events: None,
            allow_event
        }
    }
}

//don't modify any data, just read it
//useful for rendering, animating, etc
pub trait PassiveSystem {
    fn update(
        &mut self,
        component_manager: &ComponentManager,
        space: &Space,
        window: &mut Window,
        delta_time: f32 // real elapsed time
    ) -> Option<Vec<Event>>;
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