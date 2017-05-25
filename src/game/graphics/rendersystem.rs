use ecs::*;
use graphics::*;
use util::Point;
use game::graphics::TileComponent;
use game::Blackboard;

pub struct RenderSystem {
    _draw_target: DrawTarget,
    _width: u32,
    _height: u32,
    _window_x: u32,
    _window_y: u32,
    pub world_x: i32,
    pub world_y: i32
}

impl RenderSystem {
    pub fn new(
        width:u32,
        height:u32,
        window_x:u32,
        window_y:u32
    ) -> Self {
        Self {
            _draw_target: DrawTarget::new(width, height),
            _width: width,
            _height: height,
            _window_x: window_x,
            _window_y: window_y,
            world_x: 0,
            world_y: 0
        }
    }
}

impl PassiveSystem for RenderSystem {
    fn update(
        &mut self,
        blackboard: &mut Blackboard,
        component_manager: &ComponentManager,
        space: &Space,
        glyphbatch: &mut GlyphBatch,
        window: &mut Window,
        delta_time: f32 
    ) {
        match blackboard.player {
            Some(player) => {
                let position_component = component_manager
                    .get::<PositionComponent>(player).unwrap();
                self.world_x = 
                    position_component.point.x - (self._width as i32) / 2;
                self.world_y = 
                    position_component.point.y - (self._height as i32) / 2;
            },
            None => {}
        }
        self._draw_target.clear();
        //TODO: hard overwrite tiles with floor tiles
        for i in 0..self._width {
            let x = self.world_x + (i as i32);
            for j in 0..self._height {
                let y = self.world_y + (j as i32);
                let entities = match space.entities_at(Point::new(x,y)) {
                    Some(vector) => vector,
                    None         => { continue; }
                };

                let mut max_depth = 0;

                for n in 0..entities.len() {
                    let index = n;
                    let entity = entities[index as usize];
                    let tile_component_option = 
                        component_manager.get::<TileComponent>(entity);
                    let tile = match tile_component_option {
                        Some(tc) => Some(tc.tile),
                        None     => None
                    };
                    let position_component_option =
                        component_manager.get::<PositionComponent>(entity);
                    let position_component = position_component_option.unwrap();
                    
                    if position_component.z >= max_depth {
                        self._draw_target.overlay_tile(
                            tile,
                            i + self._window_x,
                            j + self._window_y
                        );
                        max_depth = position_component.z;
                    }
                }
            }
        }

        glyphbatch.drawtarget.set_from_drawtarget(
            &self._draw_target,
            self._window_x,
            self._window_y
        );
    }
}