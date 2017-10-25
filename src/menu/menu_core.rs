use core_manager::CoreState;
use ecs::*;
use game::*;
use game::ui::*;
use graphics::*;
use menu::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MenuType {
    Main,
    Game,
    Config
}

pub struct MenuCore {
    _config_menu: ConfigMenu,
    _game_menu: GameMenu,
    _main_menu: MainMenu,
}

impl MenuCore {
    pub fn new() -> Self {
        Self {
            _config_menu: ConfigMenu::new(),
            _game_menu: GameMenu::new(),
            _main_menu: MainMenu::new()
        }
    }

    pub fn initialize(&mut self, blackboard: &mut Blackboard) {

    }

    pub fn update(
        &mut self,
        menu_type: MenuType,
        blackboard: &mut Blackboard,
        engine: &mut Engine,
        glyph_batch: &mut GlyphBatch,
        mouse_interface: &mut MouseInterface,
        window: &mut Window
    ) -> CoreState {
        match menu_type {
            Config => self._config_menu.update(
                blackboard,
                engine,
                glyph_batch,
                mouse_interface,
                window
            ),
            Game   => self._game_menu.update(
                blackboard,
                engine,
                glyph_batch,
                mouse_interface,
                window
            ),
            Main   => self._main_menu.update(
                blackboard,
                engine,
                glyph_batch,
                mouse_interface,
                window
            ),
            _      => CoreState::Menu(menu_type)
        }
    }
}