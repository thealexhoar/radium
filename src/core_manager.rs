use ecs::*;
use game::*;
use game::component::*;
use game::render::*;
use game::ui::*;
use graphics::*;
use menu::*;
use util::*;

const WINDOW_WIDTH:u32   = 80;
const WINDOW_HEIGHT:u32  = 45;
const GAME_WIDTH:u32     = 55;
const GAME_HEIGHT:u32    = 31;
const CONSOLE_WIDTH:u32  = GAME_WIDTH;
const CONSOLE_HEIGHT:u32 = WINDOW_HEIGHT - GAME_HEIGHT;
const INFO_WIDTH:u32     = WINDOW_WIDTH - GAME_WIDTH;
const INFO_HEIGHT:u32    = WINDOW_HEIGHT;

const TURN_MAX_TIME:u32 = 300;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CoreState {
    Game,
    Menu(MenuType)
}

pub struct CoreManager {
    _width: u32,
    _height: u32,
    _blackboard: Blackboard,
    _engine: Engine,
    _glyph_batch: GlyphBatch,
    _mouse_interface: MouseInterface,
    _scheduler: Scheduler,
    _state: CoreState,
    _window: Window,
}


impl CoreManager {
    pub fn new(width: u32, height: u32) -> Self {
        Self { 
            _width: width,
            _height: height,
            _blackboard: Blackboard::new(),
            _engine: Engine::new(),
            _glyph_batch: GlyphBatch::new(
                GlyphSet::new("assets/tileset_16x16.png", 16, 16, 1024),
                WINDOW_WIDTH, WINDOW_HEIGHT,
                width, height
            ),
            _mouse_interface: MouseInterface::new(
                GAME_WIDTH, GAME_HEIGHT,
                INFO_WIDTH, INFO_HEIGHT
            ),
            _scheduler: Scheduler::new(),
            _state: CoreState::Menu(MenuType::Main),
            _window: Window::new(width, height),
        }
    }

    pub fn run(&mut self) {
        self._blackboard.camera = Some(Camera::new(0, 0, 0, 8));
        self._blackboard.max_action_time = TURN_MAX_TIME;


        self._engine.passive_systems.push(
            Box::new(RenderSystem::new(
                GAME_WIDTH,
                GAME_HEIGHT,
                0, 0
            ))
        );
        self._engine.passive_systems.push(
            Box::new(SelectionRenderSystem::new(
                GAME_WIDTH,
                GAME_HEIGHT
            ))
        );
        self._engine.passive_systems.push(
            Box::new(ConsoleSystem::new(
                CONSOLE_WIDTH,
                CONSOLE_HEIGHT,
                0, GAME_HEIGHT
            ))
        );
        self._engine.passive_systems.push(
            Box::new(InfoSystem::new(
                INFO_WIDTH,
                INFO_HEIGHT,
                GAME_WIDTH, 0
            ))
        );

        self._engine.initialize(&mut self._blackboard);

        //test level initialization
        for i in 0..4 {
            let p = self._engine.component_manager.create_entity();
            self._engine.component_manager
                .set(p, PositionComponent::new(1 + i, 1, 0, 1));
            self._engine.component_manager.set(p, TileComponent::new(
                Tile::new(
                    Some(RGBColor::new_from_rgb(155, 200, 255)),
                    None,
                    160
                )
            ));
            self._engine.component_manager.set(p, ColliderComponent::new(1));

            self._engine.space.add_entity_at(p, Point::new(1 + i, 1, 0));
            self._blackboard.player_entities.insert(p);
            self._scheduler.push_entity(p, 0);
        }

        let tile_fg = RGBColor::new_from_rgb(30, 30, 30);
        let tile_bg = RGBColor::new_from_rgb(0, 0, 0);

        let floor_tile = Tile::new(
            Some(tile_fg),
            Some(tile_bg),
            663
        );
        let wall_tile = Tile::new(
            Some(RGBColor::white()),
            Some(tile_bg),
            480
        );
        for l in 0..2 {
            for i in 0..40 {
                for j in 0..25 {
                    let entity = self._engine.component_manager.create_entity();
                    self._engine.component_manager.set(
                        entity,
                        PositionComponent::new(i, j, l, 0)
                    );
                    let mut tile = floor_tile;
                    if i == 0 || j == 0 {
                        tile = wall_tile;
                        self._engine.component_manager.set(
                            entity,
                            ColliderComponent::new(1)
                        );
                    } else if l == 0 {
                        self._engine.component_manager.set(
                            entity,
                            FloorComponent::new(false, false)
                        );
                    }
                    else {
                        continue;
                    }
                    self._engine.component_manager.set(
                        entity,
                        TileComponent::new(tile)
                    );
                    self._engine.space.add_entity_at(entity, Point::new(i, j, l));
                }
            }
        }
        for i in 0..20 {
            for j in 0..10 {
                let entity = self._engine.component_manager.create_entity();
                self._engine.component_manager.set(
                    entity,
                    PositionComponent::new(i, j, 2, 0)
                );
                self._engine.component_manager.set(
                    entity,
                    TileComponent::new(wall_tile)
                );
                self._engine.space.add_entity_at(entity, Point::new(i, j, 2));
            }
        }

        self._blackboard.current_entity = match self._scheduler.pop_entity() {
            Some((e, dt)) => Some(e),
            None          => None
        };


        let mut game_core = GameCore::new();
        let mut menu_core = MenuCore::new(
            Box::new(
                Label::new(
                    0, 0,
                    String::from("Config Menu"),
                    RGBColor::yellow(),
                    None
                )
            ),
            Box::new(
                Label::new(
                    0, 0,
                    String::from("Game Menu"),
                    RGBColor::yellow(),
                    None
                )
            ),
            Box::new(
                Label::new(
                    0, 0,
                    String::from("Main Menu"),
                    RGBColor::yellow(),
                    None
                )
            )
        );

        while self._window.is_open() {
            let mut next_state = match self._state {
                CoreState::Game => game_core.update(
                        &mut self._blackboard,
                        &mut self._engine,
                        &mut self._glyph_batch,
                        &mut self._mouse_interface,
                        &mut self._scheduler,
                        &mut self._window
                ),
                CoreState::Menu(menu_type) => menu_core.update(
                        menu_type,
                        &mut self._blackboard,
                        &mut self._engine,
                        &mut self._glyph_batch,
                        &mut self._mouse_interface,
                        &mut self._window
                ), 
            };

            self._state = next_state;
        }
    }
}