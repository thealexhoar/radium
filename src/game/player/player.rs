use ecs::*;
use behavior::*;
use game::action::construct_move_event;
use util::Point;

#[derive(Copy, Clone)]
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

pub struct PlayerBehavior {
    
}

impl PlayerBehavior{
    pub fn new() -> Self { Self{ } }
}

impl Behavior for PlayerBehavior{
    fn execute(
        &mut self,
        entity: Entity,
        component_manager: &mut ComponentManager,
        space: &mut Space
    ) -> (Result, Option<Vec<Event>>) {
        let player_component = component_manager
            .get_mut::<PlayerComponent>(entity).unwrap();
        
        match player_component.next_action {
            Some(action) => match action {
                _ => {}
            },
            None         => {/*TODO: Figure out what goes here */}
        };

        (Result::Success, Some(vec![
            construct_move_event(entity, Point::new(1,0), 0)
            ]))
    }
}
