use graphics::Tile;
use ecs::Component;

pub struct TileComponent {
    pub tile: Tile,
    pub z: u32
}
impl Component for TileComponent {}
impl TileComponent {
    pub fn new(tile: Tile, z:u32) -> Self {
        Self { tile, z }
    }
}