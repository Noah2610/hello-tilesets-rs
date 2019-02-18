use std::time::Duration;

use ggez::{
    Context,
    GameResult,
    graphics::{ self, Image, DrawParam },
    nalgebra::{ Point2, Vector2 },
    event::KeyCode,
};

use crate::Size;
use crate::Object;

const STEP: f32 = 25.0;
const MAX_VEL_X: f32 = 250.0;
const MAX_VEL_Y: f32 = 250.0;

#[derive(PartialEq)]
enum MoveDir {
    Up,
    Down,
    Left,
    Right,
    None,
}

pub struct Player {
    name:       String,
    pos:        Point2<f32>,
    velocity:   Point2<f32>,
    size:       Size<f32>,
    image:      Option<Image>,
    moved_dirs: Vec<MoveDir>,
}

impl Player {
    pub fn new<T: ToString>( name: T,
                             pos:  Point2<f32>,
                             size: Size<f32> ) -> Self {
        Self {
            name:       name.to_string(),
            pos,
            velocity:   Point2::new(0.0, 0.0),
            size,
            image:      None,
            moved_dirs: Vec::new(),
        }
    }

    pub fn init(&mut self, ctx: &mut Context) -> GameResult {
        self.image = Some(Image::new(ctx, "/player.png")?);
        Ok(())
    }

    pub fn handle_input(&mut self, keycodes: &Vec<KeyCode>) {
        use self::KeyCode as Key;
        use self::MoveDir as Dir;
        for keycode in keycodes {
            match keycode {
                Key::W => self.incr_vel(Dir::Up),
                Key::S => self.incr_vel(Dir::Down),
                Key::A => self.incr_vel(Dir::Left),
                Key::D => self.incr_vel(Dir::Right),
                _      => (),
            }
        }
    }

    fn handle_move(&mut self, dt: &Duration) {
        let dt_secs = dt.as_secs() as f32 + dt.subsec_millis() as f32 * 0.001;
        self.pos.x += self.velocity.x * dt_secs;
        self.pos.y += self.velocity.y * dt_secs;
    }

    fn incr_vel(&mut self, dir: MoveDir) {
        use self::MoveDir::*;
        if let Some(point) = match dir {
            Up    => Some(Point2::new(0.0, -STEP)),
            Down  => Some(Point2::new(0.0,  STEP)),
            Left  => Some(Point2::new(-STEP, 0.0)),
            Right => Some(Point2::new( STEP, 0.0)),
            None  => Option::None,
        } {
            self.moved_dirs.push(dir);
            self.velocity = Point2::new(self.velocity.x + point.x, self.velocity.y + point.y);
            if self.velocity.x > MAX_VEL_X {
                self.velocity.x = MAX_VEL_X;
            } else if self.velocity.x < -MAX_VEL_X {
                self.velocity.x = -MAX_VEL_X;
            }
            if self.velocity.y > MAX_VEL_Y {
                self.velocity.y = MAX_VEL_Y;
            } else if self.velocity.y < -MAX_VEL_Y {
                self.velocity.y = -MAX_VEL_Y;
            }
        }
    }

    fn handle_decr_vel(&mut self) {
        use self::MoveDir::*;
        // x
        if self.velocity.x > 0.0 && !self.moved_dirs.contains(&Right) {
            self.velocity.x -= STEP;
            if self.velocity.x < 0.0 { self.velocity.x = 0.0; }
        } else if self.velocity.x < 0.0 && !self.moved_dirs.contains(&Left) {
            self.velocity.x += STEP;
            if self.velocity.x > 0.0 { self.velocity.x = 0.0; }
        }
        // y
        if self.velocity.y > 0.0 && !self.moved_dirs.contains(&Down) {
            self.velocity.y -= STEP;
            if self.velocity.y < 0.0 { self.velocity.y = 0.0; }
        } else if self.velocity.y < 0.0 && !self.moved_dirs.contains(&Up) {
            self.velocity.y += STEP;
            if self.velocity.y > 0.0 { self.velocity.y = 0.0; }
        }
    }

    fn scale(&self) -> Option<Vector2<f32>> {
        if let Some(image) = &self.image {
            Some(Vector2::new(
                    self.size.w as f32 / image.width()  as f32,
                    self.size.h as f32 / image.height() as f32
            ))
        } else {
            None
        }
    }

    pub fn update(&mut self, ctx: &mut Context, dt: &Duration) -> GameResult {
        self.handle_move(dt);
        self.handle_decr_vel();
        self.moved_dirs.clear();
        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        if let Some(image) = &self.image {
            let p = DrawParam::new()
                .dest(self.pos)
                .scale(self.scale().unwrap());
            graphics::draw(ctx, image, p)?;
        }
        Ok(())
    }
}

impl From<&Object> for Player {
    fn from(obj: &Object) -> Self {
        Player::new(
            obj.name.clone(),
            obj.pos,
            obj.size.clone()
        )
    }
}
