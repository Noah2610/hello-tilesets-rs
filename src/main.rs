extern crate ggez;

mod tileset;

use ggez::{
    Context,
    GameResult,
    graphics::{ self, Image },
    event::{ self, EventHandler, KeyCode },
    timer,
};

use std::env;
use std::path;

use self::tileset::Tileset;

const WINDOW_SIZE: Size<f32> = Size {
    w: 800.0,
    h: 600.0,
};

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
    tileset: Tileset,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(Self {
            tileset: Tileset::new(
                         Image::new(ctx, "/tileset.png")?,
                         Size::new(32, 32),
                     ),
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        self.tileset.draw_n(ctx, 0)?;

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
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ggez::filesystem::mount(&mut ctx, &path, true);
    }

    let mut state = MainState::new(&mut ctx).expect("Couldn't load MainState");
    if let Err(e) = event::run(&mut ctx, &mut event_loop, &mut state) {
        eprintln!("Error: {}", e);
    }
}
