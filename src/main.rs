extern crate ggez;
extern crate json;

mod helpers;
mod tile;
mod tileset;

use std::env;
use std::path::{ Path, PathBuf };
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::time::{ Instant, Duration };

pub use self::helpers::Size;
pub use self::tile::Tile;

use ggez::{
    Context,
    GameResult,
    GameError,
    graphics::{ self, Image, DrawParam, Rect, spritebatch::SpriteBatch },
    event::{ self, EventHandler, KeyCode },
    timer,
    nalgebra::Point2,
};

use self::helpers::*;
use self::tileset::Tileset;

const WINDOW_SIZE: Size<f32> = Size {
    w: 1240.0,
    h: 800.0,
};
const STEP: f32 = 8.0;
const MAP: &'static str = "map.json";
const DEBUG_EVERY_MS: u64 = 1000;

struct MainState {
    camera:         Point2<f32>,
    keys_down:      Vec<KeyCode>,
    tilesets:       HashMap<String, Tileset>,
    tiles:          Vec<Tile>,
    last_debug:     Instant,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut level_file = File::open(format!("resources/{}", MAP))?;
        let mut json_raw = String::new();
        level_file.read_to_string(&mut json_raw)?;
        let json = json::parse(&json_raw).expect("Couldn't parse json data");

        let (tilesets, tiles) = load_json(ctx, &json)?;

        Ok(Self {
            camera:     Point2::new(0.0, 0.0),
            keys_down:  Vec::new(),
            tilesets,
            tiles,
            last_debug: Instant::now(),
        })
    }

    fn debug(&mut self, ctx: &mut Context) {
        let fps = timer::fps(ctx);
        println!("FPS: {}", fps);
        self.last_debug = Instant::now();
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
        let now = Instant::now();
        if now - self.last_debug >= Duration::from_millis(DEBUG_EVERY_MS) {
            self.debug(ctx);
        }

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
