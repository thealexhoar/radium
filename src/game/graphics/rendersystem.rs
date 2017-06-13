use ecs::*;
use graphics::*;
use util::Point;
use game::graphics::TileComponent;
use game::Blackboard;
use game::graphics::Camera;

pub struct RenderSystem {
    draw_target: DrawTarget,
    width: u32,
    height: u32,
    window_x: u32,
    window_y: u32,
    current_depth: u32,
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
            draw_target: DrawTarget::new(width, height),
            width: width,
            height: height,
            window_x: window_x,
            window_y: window_y,
            current_depth: 0,
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
                self.world_x = camera.x - (self.width as i32) / 2;
                self.world_y = camera.y - (self.height as i32) / 2;
                self.current_depth = camera.z;
            },
            None => {}
        }
        self.draw_target.clear();
        for i in 0..self.width {
            let x = self.world_x + (i as i32);
            for j in 0..self.height {
                let y = self.world_y + (j as i32);

                let mut max_depth = 0;
                let mut max_subdepth = 0;

                for k in 0..(self.current_depth+1) {
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
                            Some(tc) => tc.tile,
                            None     => {continue;}
                        };
                        let position_component_option =
                            component_manager.get::<PositionComponent>(entity);
                        let position_component =
                            position_component_option.unwrap();

                        //render different color for lower items

                        let step = self.current_depth - k;
                        if step > 0 {
                            tile.fg_color = match tile.fg_color {
                                Some(color) => Some(
                                    color.pow_stepdown(step, 1)
                                ),
                                None        => None
                            };
                            tile.bg_color = Some(Color::black());
                        }

                        let highest =
                            position_component.point.z > max_depth
                            || ( position_component.point.z == max_depth
                                && position_component.sub_z >= max_subdepth);

                        let height_allowed =
                            position_component.point.z <= self.current_depth;

                        if height_allowed {
                            if highest {
                                self.draw_target.overlay_tile(
                                    Some(tile),
                                    i + self.window_x,
                                    j + self.window_y
                                );
                                max_depth = position_component.point.z;
                                max_subdepth = position_component.sub_z;
                            }
                            else {
                                self.draw_target.overlay_tile_soft(
                                    Some(tile),
                                    i + self.window_x,
                                    j + self.window_y
                                );
                            }
                        }
                    }
                }
            }
        }

        glyphbatch.drawtarget.set_from_drawtarget(
            &self.draw_target,
            self.window_x,
            self.window_y
        );
    }
}