use std::collections::HashMap;

use ggez::{
    Context,
    GameResult,
    GameError,
    graphics::{ Image, },
    nalgebra::Point2,
};

use crate::Tile;
use crate::tileset::Tileset;

pub struct Size<T> {
    pub w: T,
    pub h: T,
}

impl<T> Size<T> {
    pub fn new(w: T, h: T) -> Self {
        Self { w, h }
    }
}

pub fn load_json(ctx: &mut Context, json: &json::JsonValue) -> GameResult<(HashMap<String, Tileset>, Vec<Tile>)> {
    let mut tilesets = HashMap::new();
    let mut tiles    = Vec::new();

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

    Ok((tilesets, tiles))
}
