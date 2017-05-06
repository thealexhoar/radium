use ecs::{Event, Component};

pub enum PlayerAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight
}

pub struct PlayerComponent {
    pub next_action: Option<PlayerAction>
}

impl Component for PlayerComponent {}

impl PlayerComponent {
    pub fn new() -> Self { Self { next_action: None } }
}