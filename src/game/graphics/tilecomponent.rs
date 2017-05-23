use graphics::Tile;
use ecs::Component;

pub struct TileComponent {
    pub tile: Tile
}

impl Component for TileComponent {}

impl TileComponent {
    pub fn new(tile: Tile) -> Self {
        Self { tile }
    }
}