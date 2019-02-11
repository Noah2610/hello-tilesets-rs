use ggez::{
    Context,
    GameResult,
    graphics::{ self, Image, DrawParam, Rect, spritebatch::SpriteBatch, FilterMode },
    nalgebra::Point2,
};

use crate::Size;
use crate::Tile;

pub struct Tileset {
    image_size:  Size<u16>,
    spritebatch: SpriteBatch,
    tile_size:   Size<usize>,
    queued:      usize,
}

impl Tileset {
    pub fn new(image: Image, tile_size: Size<usize>) -> Self {
        let image_size = Size::new(image.width(), image.height());
        let mut spritebatch = SpriteBatch::new(image);
        spritebatch.set_filter(FilterMode::Nearest);
        Self {
            image_size,
            spritebatch,
            tile_size,
            queued: 0,
        }
    }

    pub fn queue_tile(&mut self, ctx: &mut Context, tile: &Tile, offset: &Point2<f32>) {
        let dest = Point2::new(
            tile.pos.x + offset.x,
            tile.pos.y + offset.y
        );
        let rect = self.rect_for_n(tile.id);
        let param = DrawParam::new()
            .dest(dest)
            .src(rect);
        self.spritebatch.add(param);
        self.queued += 1;
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        if self.queued == 0 { return Ok(()); }
        graphics::draw(ctx, &self.spritebatch, DrawParam::new())?;
        self.spritebatch.clear();
        self.queued = 0;
        Ok(())
    }

    fn rect_for_n(&self, idx: usize) -> Rect {
        let image_w = self.image_size.w;
        let image_h = self.image_size.h;

        //let tiles_per_col = image_h as usize / self.tile_size.h;
        let tiles_per_row = image_w as usize / self.tile_size.w;

        let row = idx / tiles_per_row;
        let col = idx - row * tiles_per_row;

        let x = self.tile_size.w as usize * col;
        let y = self.tile_size.h as usize * row;
        let w = self.tile_size.w;
        let h = self.tile_size.h;

        let rel_x = x as f32 / image_w as f32;
        let rel_y = y as f32 / image_h as f32;
        let rel_w = w as f32 / image_w as f32;
        let rel_h = h as f32 / image_h as f32;

        Rect::new(rel_x, rel_y, rel_w, rel_h)
    }
}
