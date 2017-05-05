use util::Point;

pub trait Component : 'static{
    //TODO: add a serialize function
}

//core components
//TODO: find a better home for these little guys?

//position
pub struct PositionComponent {
    pub point: Point
}
impl Component for PositionComponent {}

//turn-taking
//perhaps move to behavior?
pub struct TurnComponent;
impl Component for TurnComponent {}


