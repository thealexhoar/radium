use graphics::Tile;
use ecs::Component;

pub struct TileComponent {
    pub tile: Tile
}
impl Component for TileComponent {}

pub struct ZComponent {
    pub z: u32
}
impl Component for ZComponent {}