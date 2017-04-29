
use sfml::system::Vector2u;
use sfml::graphics::{Texture, IntRect};

use std::cmp::min;

pub struct GlyphSet {
    pub texture: Texture,
    pub sub_rects: Vec<IntRect>,
    _tile_size: Vector2u
}

impl GlyphSet {
    pub fn new(
        filename:&str, 
        tile_width:u32,
        tile_height:u32, 
        tile_count:u32
    ) -> GlyphSet {
        let texture = match Texture::from_file(filename) {
            Some(texture) => texture,
            None          => panic!("Couldn't load texture at {}", filename)
        };
        let tiles = min(tile_width * tile_height, tile_count);
        let tiled_width = texture.size().x / tile_width;
        let tiled_height = texture.size().y / tile_height;

        GlyphSet {
            texture: texture,
            sub_rects: 
                (0..tile_count)
                .map(
                    |i| -> IntRect {
                        let x = i % tiled_width;
                        let y = i / tiled_width;
                        IntRect::new(
                            (x * tile_width) as i32,
                            (y * tile_height) as i32,
                            tile_width as i32,
                            tile_height as i32
                        )
                    }
                )
                .collect(),
            _tile_size:Vector2u::new(tile_width, tile_height)
        }
    }

    pub fn tile_size(&self) -> Vector2u {
        self._tile_size
    }
}
