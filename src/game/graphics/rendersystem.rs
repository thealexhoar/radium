use ecs::*;
use graphics::*;
use util::Point;
use game::graphics::TileComponent;
use game::Blackboard;
use game::graphics::Camera;

pub struct RenderSystem {
    _draw_target: DrawTarget,
    _width: u32,
    _height: u32,
    _window_x: u32,
    _window_y: u32,
    _current_depth: u32,
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
            _current_depth: 0,
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
        match blackboard.camera {
            Some(ref camera) => {
                self.world_x = camera.x - (self._width as i32) / 2;
                self.world_y = camera.y - (self._height as i32) / 2;
                self._current_depth = camera.z;
            },
            None => {}
        }
        self._draw_target.clear();
        for i in 0..self._width {
            let x = self.world_x + (i as i32);
            for j in 0..self._height {

                let mut max_depth = 0;
                let mut max_subdepth = 0;

                for k in 0..(self._current_depth+1) {
                    let y = self.world_y + (j as i32);
                    let entities = match space.entities_at(Point::new(x,y,k)) {
                        Some(vector) => vector,
                        None         => { continue; }
                    };


                    for n in 0..entities.len() {
                        let index = n;
                        let entity = entities[index as usize];
                        let tile_component_option = 
                            component_manager.get::<TileComponent>(entity);
                        let mut tile = match tile_component_option {
                            Some(tc) => Some(tc.tile),
                            None     => None
                        };
                        let position_component_option =
                            component_manager.get::<PositionComponent>(entity);
                        let position_component = 
                            position_component_option.unwrap();


                        let highest =
                            position_component.point.z >= max_depth 
                            && position_component.sub_z >= max_subdepth;

                        let height_allowed =
                            position_component.point.z <= self._current_depth;
                    
                        if height_allowed {
                            if highest {
                                self._draw_target.overlay_tile(
                                    tile,
                                    i + self._window_x,
                                    j + self._window_y
                                );
                                max_depth = position_component.point.z;
                                max_subdepth = position_component.sub_z;
                            }
                            else {
                                self._draw_target.overlay_tile_soft(
                                    tile,
                                    i + self._window_x,
                                    j + self._window_y
                                );
                            }
                        }
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