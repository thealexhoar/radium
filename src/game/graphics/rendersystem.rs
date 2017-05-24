use ecs::*;
use graphics::*;
use util::Point;
use game::graphics::TileComponent;

pub struct RenderSystem {
    _width: u32,
    _height: u32,
    _window_x: u32,
    _window_y: u32,
    pub world_x: i32,
    pub world_y: i32,
    _glyphbatch: GlyphBatch
}

impl RenderSystem {
    pub fn new(
        width:u32,
        height:u32,
        window_x:u32,
        window_y:u32,
        glyphbatch: GlyphBatch
    ) -> Self {
        Self {
            _width: width,
            _height: height,
            _window_x: window_x,
            _window_y: window_y,
            world_x: 0,
            world_y: 0,
            _glyphbatch: glyphbatch
        }
    }
}

impl PassiveSystem for RenderSystem {
    fn update(
        &mut self,
        component_manager: &ComponentManager,
        space: &Space,
        window: &mut Window,
        delta_time: f32 
    ) {
        //TODO: hard overwrite tiles with floor tiles
        for i in 0..self._width {
            let x = self.world_x + i as i32;
            for j in 0..self._height {
                let y = self.world_y + j as i32;
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
                        self._glyphbatch.drawtarget.overlay_tile(
                            tile,
                            i + self._window_x,
                            j + self._window_y
                        );
                        max_depth = position_component.z;
                    }
                }
            }
        }
        
        window.clear();
        self._glyphbatch.flush_tiles();
        window.draw_glyphbatch(&self._glyphbatch);
    }
}