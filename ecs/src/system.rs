use Engine;
use Event;

pub trait PassiveSystem {
    fn update(&mut self, engine:&Engine);
}

pub trait ReactiveSystem {
    fn update(&mut self, engine:&Engine, event:Event);
}

pub trait ContinuousSystem {
    fn update(&mut self, engine:&Engine, delta_time:u32);
}