extern crate ggez;
extern crate json;

mod tileset;

use std::env;
use std::path::{ Path, PathBuf };
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

use ggez::{
    Context,
    GameResult,
    GameError,
    graphics::{ self, Image, DrawParam, Rect, spritebatch::SpriteBatch },
    event::{ self, EventHandler, KeyCode },
    timer,
    nalgebra::Point2,
};

use self::tileset::Tileset;

const WINDOW_SIZE: Size<f32> = Size {
    w: 1240.0,
    h: 800.0,
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

pub struct Tile {
    pub id:      usize,
    pub pos:     Point2<f32>,
    pub tileset: String,
}

impl Tile {
    pub fn new(id: usize, pos: Point2<f32>, tileset: String) -> Self {
        Self { id, pos, tileset }
    }
}

struct MainState {
    camera:         Point2<f32>,
    keys_down:      Vec<KeyCode>,
    tilesets:       HashMap<String, Tileset>,
    tiles:          Vec<Tile>,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut tilesets = HashMap::new();
        let mut tiles = Vec::new();

        let mut level_file = File::open("resources/map.json")?;
        let mut json_raw = String::new();
        level_file.read_to_string(&mut json_raw)?;
        let json = json::parse(&json_raw).expect("Couldn't parse json data");

        if json.has_key("tilesets") {
            for (name, data) in json["tilesets"].entries() {
                let image = Image::new(ctx, format!("/{}", data["image_filename"].as_str().expect("Parse string")))?;
                let tile_size;
                if data.has_key("tile_size") {
                    tile_size = Size::new(
                        data["tile_size"]["w"].as_usize().expect("Parse usize"),
                        data["tile_size"]["h"].as_usize().expect("Parse usize")
                    );
                } else { return Err(GameError::ResourceLoadError("Tileset JSON doesn't have key `tile_size`".to_string())); }
                let tileset = Tileset::new(image, tile_size);
                tilesets.insert(name.to_string(), tileset);
            }
        } else { return Err(GameError::ResourceLoadError("JSON file doesn't have key `tilesets`".to_string())); }

        if json.has_key("tiles") {
            for data in json["tiles"].members() {
                let id;
                if data.has_key("id") {
                    id = data["id"].as_usize().expect("Parse usize");
                } else { return Err(GameError::ResourceLoadError("Tile JSON doesn't have key `id`".to_string())); }
                let pos;
                if data.has_key("pos") {
                    pos = Point2::new(
                        data["pos"]["x"].as_f32().expect("Parse f32"),
                        data["pos"]["y"].as_f32().expect("Parse f32")
                    );
                } else { return Err(GameError::ResourceLoadError("Tile JSON doesn't have key `pos`".to_string())); }
                let tileset;
                if data.has_key("tileset") {
                    tileset = data["tileset"].as_str().expect("Parse usize");
                } else { return Err(GameError::ResourceLoadError("Tile JSON doesn't have key `tileset`".to_string())); }
                let tile = Tile::new(id, pos, tileset.to_string());
                tiles.push(tile);
            }
        } else {
            return Err(GameError::ResourceLoadError("JSON file doesn't have key `tiles`".to_string()));
        }

        Ok(Self {
            camera:    Point2::new(0.0, 0.0),
            keys_down: Vec::new(),
            tilesets,
            tiles,
        })
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
                KeyCode::W => self.camera.y += STEP,
                KeyCode::S => self.camera.y -= STEP,
                KeyCode::A => self.camera.x += STEP,
                KeyCode::D => self.camera.x -= STEP,
                _          => (),
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        for tile in &self.tiles {
            let tileset = self.tilesets.get_mut(&tile.tileset).expect("Should find `Tileset` for `Tile`");
            tileset.queue_tile(ctx, tile, &self.camera);
        }
        for (_name, tileset) in &mut self.tilesets {
            tileset.draw(ctx)?;
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
