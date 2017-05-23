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
impl PositionComponent {
    pub fn new(x:i32, y:i32) -> Self {
        Self { point: Point::new(x, y) }
    }
}


