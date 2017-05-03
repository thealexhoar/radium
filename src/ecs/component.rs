use util::Point;

pub trait Component : 'static{
    //TODO: add a serialize function
}

//core components
//TODO: find a better home for these little guys?

pub struct PositionComponent {
    pub point: Point
}
impl Component for PositionComponent {}

pub struct TurnComponent;
impl Component for TurnComponent {}


