extern crate sfml;

pub use color::Color as Color;
pub use drawtarget::DrawTarget as DrawTarget;
pub use glyphbatch::GlyphBatch as GlyphBatch;
pub use glyphset::GlyphSet as GlyphSet;
pub use tile::Tile as Tile;
pub use tile::TILE_ID_MAX_VALUE as TILE_ID_MAX_VALUE;


pub mod color;
pub mod drawtarget;
pub mod glyphbatch;
pub mod glyphset;
pub mod tile;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
