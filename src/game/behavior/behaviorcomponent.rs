use ecs::Component;
use behavior::{Behavior};

pub struct BehaviorComponent {
    pub behavior_box: Box<Behavior>
}

impl Component for BehaviorComponent {}
impl BehaviorComponent {
    pub fn new<T:Behavior + Sized + 'static>(behavior:T) -> Self {
        Self { behavior_box: Box::new(behavior) }
    }
}