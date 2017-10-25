use ecs::Component;

pub struct FloorComponent {
    pub allow_up: bool,
    pub allow_down: bool
}
impl Component for FloorComponent {}

impl FloorComponent {
    pub fn new(allow_up: bool, allow_down: bool) -> Self {
        Self { allow_up, allow_down }
    }
}