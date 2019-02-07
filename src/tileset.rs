use ggez::{
    Context,
    GameResult,
    graphics::{ self, Image, DrawParam, Rect },
    nalgebra::Point2,
};

use crate::Size;

pub struct Tileset {
    image:     Image,
    tile_size: Size<usize>,
}

impl Tileset {
    pub fn new(image: Image, tile_size: Size<usize>) -> Self {
        Self {
            image,
            tile_size,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let p = DrawParam::new()
            .dest(Point2::new(0.0, 0.0))
            .src(Rect::new(0.0, 0.0, 0.25, 0.25));
        graphics::draw(ctx, &self.image, p)
    }

    pub fn draw_n(&mut self, ctx: &mut Context, idx: usize) -> GameResult {
        let rect = self.rect_for_n(idx);
        let p = DrawParam::new()
            .dest(Point2::new(0.0, 0.0))
            .src(rect);
        graphics::draw(ctx, &self.image, p)
    }

    fn rect_for_n(&self, idx: usize) -> Rect {
        let image_w = self.image.width();
        let image_h = self.image.height();

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
