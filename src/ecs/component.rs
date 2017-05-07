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

//turn-taking
//perhaps move to behavior?
pub struct TurnComponent {
    initialized: bool
}
impl Component for TurnComponent {}
impl TurnComponent {
    pub fn new() -> Self {
        Self { initialized: false }
    }

    pub fn init(&mut self) {
        self.initialized = true;
    }
}


