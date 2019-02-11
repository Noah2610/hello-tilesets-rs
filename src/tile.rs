use ggez::nalgebra::Point2;

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
