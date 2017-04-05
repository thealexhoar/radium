use Engine;
use Event;
use ComponentManager;

pub struct EventResult{
    pub resulting_events: Vec<Event>,
    pub allow_event: bool
}

//don't modify any data, just read it
pub trait PassiveSystem {
    fn update(&mut self, engine:&Engine) -> EventResult;
}

//respond to actions 
pub trait ReactiveSystem {
    fn update(
        &mut self,
        component_manager: &mut ComponentManager,
        event:Event
    ) -> EventResult;
}

//run continuously to handle continuous processes
//ie animations
pub trait ContinuousSystem {
    fn update(
        &mut self,
        component_manager: &mut ComponentManager,
        delta_time: f32 // real elapsed time
    ) -> EventResult;
}