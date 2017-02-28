extern crate sfml;

use {DrawTarget, GlyphSet, Tile, Color, TILE_ID_MAX_VALUE};

use sfml::system::{Vector2u, Vector2f};
use sfml::graphics::{
    BlendMode, Transform, PrimitiveType, 
    Image, Texture, IntRect, RenderWindow, 
    RenderStates, RenderTarget, VertexArray, Vertex};
use sfml::graphics::Color as SFColor;

pub struct GlyphBatch {
    _tile_size: Vector2u,
    _tiled_dimensions: Vector2u,
    _pixel_dimensions: Vector2u,
    _big_offset: Vector2u,
    _small_offset: Vector2u,
    _fg_vertices: VertexArray,
    _bg_vertices: VertexArray,
    _glyphset: GlyphSet,
    _null_texture: Texture,
    pub drawtarget: DrawTarget
}

impl GlyphBatch {
    pub fn new(
        glyphset:GlyphSet,
        tiled_width:u32, tiled_height:u32,
        pixel_width:u32, pixel_height:u32
    ) -> GlyphBatch {
        let null_img = match Image::new_from_color(1, 1, &SFColor::white()) {
            Some(img) => img,
            None      => panic!("Couldn't create Image")
        };
        let vertex_count = tiled_width * tiled_height * 4;

        let mut glyphbatch = GlyphBatch{
            _tile_size: Vector2u::new(0, 0),
            _tiled_dimensions: Vector2u::new(tiled_width, tiled_height),
            _pixel_dimensions: Vector2u::new(0, 0),
            _big_offset: Vector2u::new(0, 0),
            _small_offset: Vector2u::new(0, 0),
            _fg_vertices: match VertexArray::new_init(
                PrimitiveType::sfQuads, vertex_count) 
            {
                Some(vertex_array) => vertex_array,
                None               => panic!("Couldn't create VertexArray")
            },
            _bg_vertices: match VertexArray::new_init(
                PrimitiveType::sfQuads, vertex_count)
            {
                Some(vertex_array) => vertex_array,
                None               => panic!("Couldn't create VertexArray")
            },
            _glyphset: glyphset,
            drawtarget: DrawTarget::new(tiled_width, tiled_height),
            _null_texture: match Texture::new_from_image(&null_img) {
                Some(texture) => texture,
                None          => panic!("Couldn't create Texture")
            }
        };
        glyphbatch.set_pixel_resolution(pixel_width, pixel_height);

        glyphbatch
    }

    pub fn set_pixel_resolution(
        &mut self, 
        pixel_width:u32, 
        pixel_height:u32
    ) {
        let true_tile_size = Vector2f::new(
            pixel_width as f32 / self._tiled_dimensions.x as f32, 
            pixel_height as f32 / self._tiled_dimensions.y as f32
            );
        self._pixel_dimensions.x = pixel_width;
        self._pixel_dimensions.y = pixel_height;
        self._tile_size.x = true_tile_size.x as u32;
        self._tile_size.y = true_tile_size.y as u32;

        let extra_space = Vector2u::new(
            pixel_width - self._tiled_dimensions.x * self._tile_size.x,
            pixel_height - self._tiled_dimensions.y * self._tile_size.y
        );

        self._small_offset.x = extra_space.x / self._tiled_dimensions.x / 2;
        self._small_offset.y = extra_space.y / self._tiled_dimensions.y / 2;

        self._big_offset.x = (
            extra_space.x 
            - self._small_offset.x * self._tiled_dimensions.x
            ) / 2;
        self._big_offset.y = (
            extra_space.y 
            - self._small_offset.y * self._tiled_dimensions.y
            ) / 2;
    }

    pub fn render(&mut self, window:&mut RenderWindow) {
        let mut fg_renderstates = RenderStates::new (
                BlendMode::blend_alpha(),
                Transform::new_identity(),
                Some(&self._glyphset.texture),
                None
        );

        let mut bg_renderstates = RenderStates::new (
                BlendMode::blend_alpha(),
                Transform::new_identity(),
                Some(&self._null_texture),
                None
        );

        window.draw_with_renderstates(
            &self._bg_vertices, 
            &mut bg_renderstates
        );
        window.draw_with_renderstates(
            &self._fg_vertices, 
            &mut fg_renderstates
        );

    }

    pub fn flush_tiles(&mut self) {
        for x in 0..self.drawtarget.width() {
            for y in 0..self.drawtarget.height() {
                match self.drawtarget.get_tile(x, y) {
                    Some(tile) => self.set_vertices(tile, x, y),
                    None       => self.set_vertices(
                        Tile::new(None, None, 0),
                        x, y
                    ) 
                };
            }
        }
    }

    fn set_vertices(&mut self, tile:Tile, x:u32, y:u32) {
        if tile.tile_id > TILE_ID_MAX_VALUE {
            return;
        }

        match tile.fg_color {
            Some(color) => self.set_tile_fg_vertices(
                tile.tile_id,
                color, 
                x, y
            ),
            None        => self.set_tile_fg_vertices(0, Color::clear(), x, y)
        }

        match tile.bg_color {
            Some(color) => self.set_tile_bg_vertices(color, x, y),
            None        => self.set_tile_bg_vertices(Color::clear(), x, y)
        }
    }

    fn set_tile_fg_vertices(
        &mut self,
        tile_id:u32,
        color:Color,
        x:u32, y:u32
    ) {
        let base_index = (x + y * self._tiled_dimensions.x) * 4;
        let source_rect = self._glyphset.sub_rects[tile_id as usize];

        let mut next_position:Vector2f;
        let mut next_tex_coords:Vector2f;
        let mut next_color:SFColor;

        
        for i in 0..2 {
            for j in 0..2 {
                let index = match j {
                    0 => base_index + i,
                    1 => base_index + 3 - i,
                    _ => 0 //never reached
                };

                next_position = self.vertex_position(x + i, y + j);
                next_tex_coords = Vector2f::new(
                    (source_rect.left 
                    + source_rect.width * (i as i32)) as f32, 
                    (source_rect.top 
                    + source_rect.height * (j as i32)) as f32
                );
                next_color = GlyphBatch::color_to_sf_color(color);
                
                *self._fg_vertices.get_vertex(index) = Vertex::new(
                    &next_position,
                    &next_color,
                    &next_tex_coords
                );
            }
        }
    }

    fn set_tile_bg_vertices(&mut self, color:Color, x:u32, y:u32) {
        let base_index = (x + y * self._tiled_dimensions.x) * 4;

        let mut next_position:Vector2f;
        let mut next_tex_coords:Vector2f;
        let mut next_color:SFColor;

        
        for i in 0..2 {
            for j in 0..2 {
                let index = match j {
                    0 => base_index + i,
                    1 => base_index + 3 - i,
                    _ => 0 //never reached
                };


                next_position = self.vertex_position(x + i, y + j);
                next_tex_coords = Vector2f::new(
                    i as f32,
                    j as f32
                );
                next_color = GlyphBatch::color_to_sf_color(color);
                
                *self._bg_vertices.get_vertex(index) = Vertex::new(
                    &next_position,
                    &next_color,
                    &next_tex_coords
                );
            }
        }

    }

    fn vertex_position(&self, x:u32, y:u32) -> Vector2f {
        Vector2f::new(
            (self._big_offset.x 
            + x * (self._tile_size.x + self._small_offset.x)) as f32,
            (self._big_offset.y 
            + y * (self._tile_size.y + self._small_offset.y)) as f32
        )
    }

    fn color_to_sf_color(color:Color) -> SFColor {
        SFColor::new_rgba(color.r, color.g, color.b, color.a)
    }

}