

pub trait Component : 'static{
    //TODO: add a serialize function
}

//core components
//TODO: find a better home for these little guys?

pub struct PositionComponent {
    pub x:i32,
    pub y:i32
}
impl Component for PositionComponent {}

pub struct TurnComponent;
impl Component for TurnComponent {}


