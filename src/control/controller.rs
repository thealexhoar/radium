pub use ecs::*;

pub struct Controller {

}

pub struct ControllerComponent {
    pub controller: Controller
}

impl Component for ControllerComponent {}