extern crate ggez;
extern crate tiled;

//mod tileset;

use std::env;
use std::path::{ Path, PathBuf };
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;

use ggez::{
    Context,
    GameResult,
    graphics::{ self, Image, DrawParam, Rect },
    event::{ self, EventHandler, KeyCode },
    timer,
    nalgebra::Point2,
};
use tiled::{
    Map,
    Tileset,
    Tile,
};

const WINDOW_SIZE: Size<f32> = Size {
    w: 800.0,
    h: 600.0,
};

const STEP: f32 = 8.0;

pub struct Size<T> {
    pub w: T,
    pub h: T,
}

impl<T> Size<T> {
    pub fn new(w: T, h: T) -> Self {
        Self { w, h }
    }
}

struct MainState {
    map:            Map,
    tileset_images: HashMap<String, Image>,
    camera:         Point2<f32>,
    keys_down:      Vec<KeyCode>,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let map = tiled::parse_file(
            Path::new("resources/map_two.tmx")
        ).unwrap();

        let mut tileset_images = HashMap::new();
        for tileset in &map.tilesets {
            let path = format!("/{}", tileset.images[0].source);
            let image = Image::new(ctx, &path)?;
            tileset_images.insert(tileset.name.clone(), image);
        }

        Ok(Self {
            map,
            tileset_images,
            camera:    Point2::new(0.0, 0.0),
            keys_down: Vec::new(),
        })
    }

    fn get_tileset_and_tile_by_gid(&self, gid: u32) -> Option<(&Tileset, &Tile)> {
        for tileset in &self.map.tilesets {
            if let Some(tile) = tileset.tiles.iter().find( |t| t.id == gid ) {
                return Some((tileset, tile));
            }
        }
        None
    }

    fn draw_tile(&self, ctx: &mut Context, tileset: &Tileset, tile: &Tile, pos: Point2<u32>) -> GameResult {
        let point = Point2::new(
            (tileset.tile_width * pos.x)  as f32 + self.camera.x,
            (tileset.tile_height * pos.y) as f32 + self.camera.y
        );
        let rect = self.rect_for_tile(tileset, tile);
        let p = DrawParam::new()
            .dest(point)
            .src(rect);
        let image = self.tileset_images.get(&tileset.name).unwrap();
        graphics::draw(ctx, image, p);
        Ok(())
    }

    fn rect_for_tile(&self, tileset: &Tileset, tile: &Tile) -> Rect {
        let image = tileset.images.first().unwrap();
        let tile_size = Size::new(tileset.tile_width, tileset.tile_height);
        let id = tile.id;

        let image_w = image.width;
        let image_h = image.height;

        //let tiles_per_col = image_h as usize / self.tile_size.h;
        let tiles_per_row = image_w as u32 / tile_size.w;

        let row = id / tiles_per_row;
        let col = id - row * tiles_per_row;

        let x = tile_size.w as u32 * col;
        let y = tile_size.h as u32 * row;
        let w = tile_size.w;
        let h = tile_size.h;

        let rel_x = x as f32 / image_w as f32;
        let rel_y = y as f32 / image_h as f32;
        let rel_w = w as f32 / image_w as f32;
        let rel_h = h as f32 / image_h as f32;

        Rect::new(rel_x, rel_y, rel_w, rel_h)
    }
}

impl EventHandler for MainState {
    fn key_down_event(&mut self,
                      ctx:      &mut Context,
                      keycode:  KeyCode,
                      _keymods: event::KeyMods,
                      repeat:   bool) {
        if !repeat {
            if let KeyCode::Escape = keycode {
                ggez::quit(ctx);
            }
            if !self.keys_down.contains(&keycode) {
                self.keys_down.push(keycode);
            }
        }
    }

    fn key_up_event(&mut self,
                    ctx:      &mut Context,
                    keycode:  KeyCode,
                    _keymods: event::KeyMods,
                    ) {
        if let Some(idx) = self.keys_down.iter().position( |&k| k == keycode ) {
            self.keys_down.remove(idx);
        }
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        for keycode in &self.keys_down {
            match keycode {
                KeyCode::W      => self.camera.y += STEP,
                KeyCode::S      => self.camera.y -= STEP,
                KeyCode::A      => self.camera.x += STEP,
                KeyCode::D      => self.camera.x -= STEP,
                _               => (),
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        for layer in &self.map.layers {
            for (row, tiles) in layer.tiles.iter().enumerate() {
                for (col, id) in tiles.iter().enumerate() {
                    if let Some((tileset, tile)) = self.get_tileset_and_tile_by_gid(*id) {
                        self.draw_tile(
                            ctx,
                            &tileset,
                            &tile,
                            Point2::new(col as u32, row as u32)
                        )?;
                    }
                }
            }
        }

        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }
}

fn main() {
    let (mut ctx, mut event_loop) = ggez::ContextBuilder::new(
        "tileset-test", "Noah"
    ).window_setup(
        ggez::conf::WindowSetup::default().title("Tileset Test")
    ).window_mode(
        ggez::conf::WindowMode::default().dimensions(
            WINDOW_SIZE.w,
            WINDOW_SIZE.h
        )
    ).build().expect("Couldn't build context");

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = PathBuf::from(manifest_dir);
        path.push("resources");
        ggez::filesystem::mount(&mut ctx, &path, true);
    }

    let mut state = MainState::new(&mut ctx).expect("Couldn't load MainState");
    if let Err(e) = event::run(&mut ctx, &mut event_loop, &mut state) {
        eprintln!("Error: {}", e);
    }
}
