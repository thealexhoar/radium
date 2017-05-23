use ecs::*;
use graphics::*;

pub struct RenderSystem {
    _width: u32,
    _height: u32,
    _window_x: u32,
    _window_y: u32,
    pub world_x: u32,
    pub world_y: u32,
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
        window.clear();
        self._glyphbatch.flush_tiles();
        window.draw_glyphbatch(&self._glyphbatch);
    }
}