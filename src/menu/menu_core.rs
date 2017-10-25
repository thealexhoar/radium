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
    pub fn new(
        config_menu_object: Box<MenuObject>,
        game_menu_object: Box<MenuObject>,
        main_menu_object: Box<MenuObject>
    ) -> Self {
        Self {
            _config_menu: ConfigMenu::new(config_menu_object),
            _game_menu: GameMenu::new(game_menu_object),
            _main_menu: MainMenu::new(main_menu_object)
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
            MenuType::Config => self._config_menu.update(
                blackboard,
                engine,
                glyph_batch,
                mouse_interface,
                window
            ),
            MenuType::Game   => self._game_menu.update(
                blackboard,
                engine,
                glyph_batch,
                mouse_interface,
                window
            ),
            MenuType::Main   => self._main_menu.update(
                blackboard,
                engine,
                glyph_batch,
                mouse_interface,
                window
            ),
            _                => CoreState::Menu(menu_type)
        }
    }
}