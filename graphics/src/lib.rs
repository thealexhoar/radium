extern crate sfml;

pub use color::Color;
pub use drawtarget::DrawTarget;
pub use glyphbatch::GlyphBatch;
pub use glyphset::GlyphSet;
pub use tile::{Tile, TILE_ID_MAX_VALUE};
pub use window::{Window, Event, Events};


pub mod color;
pub mod drawtarget;
pub mod glyphbatch;
pub mod glyphset;
pub mod tile;
pub mod window;

#[cfg(test)]
mod tests;