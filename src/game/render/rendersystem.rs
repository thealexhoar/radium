use game::render::{TileComponent};
use ecs::{ComponentManager, Event, EventResult, PassiveSystem, Space};
use graphics::{DrawTarget, GlyphBatch, Tile, Color, Window};
use util::Point;

pub struct RenderSystem {
    pub glyphbatch: GlyphBatch,
    pub drawtarget: DrawTarget
}

impl RenderSystem {
    pub fn new(glyphbatch: GlyphBatch, width:u32, height:u32) -> Self {
        Self {
            glyphbatch,
            drawtarget: DrawTarget::new(width, height)
        }
    }
}

impl PassiveSystem for RenderSystem {
    fn update(
        &mut self,
        component_manager: &ComponentManager,
        space: &Space,
        window: &mut Window,
        delta_time: f32 // real elapsed time
    ) -> Option<Vec<Event>> {
        window.clear();

        'vertical: for j in 0..self.drawtarget.height() as i32 {
            'horizontal: for i in 0..self.drawtarget.width() as i32 {
                //TODO: replace with pure black sometime
                let mut draw_tile:Tile = Tile::new(
                    Some(Color::cyan()),
                    Some(Color::magenta()),
                    'X' as u32
                );

                match space.entities_at(Point::new(i, j)) {
                    Some(entities) => {
                        let mut max_z:i32 = -1;
                        'components:
                        for (index, entity) in entities.iter().enumerate() {
                            //try to get tile component
                            let tc = match component_manager
                                .get::<TileComponent>(*entity) {
                                Some(component) => component,
                                None            => {continue 'components;}
                            };
                            if tc.z as i32 > max_z {
                                max_z = tc.z as i32;
                                draw_tile = tc.tile;
                            }
                        }
                    },
                    None      => {}
                };
                self.drawtarget.set_tile(Some(draw_tile), i as u32, j as u32);
            }
        }
        self.glyphbatch.drawtarget.set_from_drawtarget(
            &self.drawtarget,
            30,
            10
        );
        self.glyphbatch.flush_tiles();
        window.draw_glyphbatch(&self.glyphbatch);
        None
    }
}