use graphics::*;

pub enum ButtonType {
    //TODO: implement
}

pub enum MouseInput {
    Tile(u32,u32),
    Button(ButtonType),
}

pub struct MouseInterface {
    game_width: u32,
    game_height: u32,
    info_width: u32,
    info_height: u32,
    old_left: bool,
    old_right: bool,
    current_left: bool,
    current_right: bool
}

impl MouseInterface {
    pub fn new(
        game_width: u32,
        game_height: u32,
        info_width: u32,
        info_height: u32,
    ) -> Self {
        Self {
            game_width,
            game_height,
            info_width,
            info_height,
            old_left: false,
            old_right: false,
            current_left: false,
            current_right: false
        }
    }

    pub fn update(&mut self, window:&Window) {
        self.old_left = self.current_left;
        self.old_right = self.current_right;
        self.current_left = window.mouse_pressed(MouseButton::Left);
        self.current_right = window.mouse_pressed(MouseButton::Right);
    }

    pub fn mouse_down(&self) -> (bool, bool) {
        (
            !self.old_left && self.current_left,
            !self.old_right && self.current_right
        )
    }
}