use std::collections::HashMap;

use ggez::{
    Context,
    GameResult,
    GameError,
    graphics::{ Image, },
    nalgebra::Point2,
};

use crate::Tile;
use crate::Object;
use crate::Tileset;

#[derive(Debug, Clone)]
pub struct Size<T> {
    pub w: T,
    pub h: T,
}

impl<T> Size<T> {
    pub fn new(w: T, h: T) -> Self {
        Self { w, h }
    }
}

pub fn load_json(
    ctx: &mut Context, tileset_json: &json::JsonValue, level_json: &json::JsonValue
) -> GameResult<(HashMap<String, Tileset>, Vec<Tile>, Vec<Object>)> {
    let mut tilesets = HashMap::new();
    let mut tiles    = Vec::new();
    let mut objects  = Vec::new();

    // Load tileset data
    for (name, data) in tileset_json.entries() {
        let image = Image::new(ctx, format!("/{}", data["image_filename"].as_str().expect("Parse string")))?;
        let tile_size;
        if data.has_key("tile_size") {
            tile_size = Size::new(
                data["tile_size"]["w"].as_usize().expect("Parse usize"),
                data["tile_size"]["h"].as_usize().expect("Parse usize")
            );
        } else { return reserr("Tileset JSON doesn't have key `tile_size`"); }
        let tileset = Tileset::new(image, tile_size);
        tilesets.insert(name.to_string(), tileset);
    }

    // Load tiles data
    if level_json.has_key("tiles") {
        for data in level_json["tiles"].members() {
            let id;
            if data.has_key("id") {
                id = data["id"].as_usize().expect("Parse usize");
            } else { return reserr("Tile JSON doesn't have key `id`"); }
            let pos;
            if data.has_key("pos") {
                pos = Point2::new(
                    data["pos"]["x"].as_f32().expect("Parse f32"),
                    data["pos"]["y"].as_f32().expect("Parse f32")
                );
            } else { return reserr("Tile JSON doesn't have key `pos`"); }
            let tileset;
            if data.has_key("tileset") {
                tileset = data["tileset"].as_str().expect("Parse usize");
            } else { return reserr("Tile JSON doesn't have key `tileset`"); }
            let tile = Tile::new(id, pos, tileset.to_string());
            tiles.push(tile);
        }
    } else {
        return reserr("JSON file doesn't have key `tiles`");
    }

    // Load objects data
    if level_json.has_key("objects") {
        for data in level_json["objects"].members() {
            let name;
            if data.has_key("name") {
                name = data["name"].as_str().expect("Parse string");
            } else { return reserr("Object JSON doesn't habe key `name`"); }
            let otype;
            if data.has_key("type") {
                otype = data["type"].as_str().expect("Parse string");
            } else { return reserr("Object JSON doesn't habe key `type`"); }
            let pos;
            if data.has_key("pos") {
                pos = (
                    data["pos"]["x"].as_f32().expect("Parse f32: pos x"),
                    data["pos"]["y"].as_f32().expect("Parse f32: pos y")
                );
            } else { return reserr("Object JSON doesn't habe key `pos`"); }
            let size;
            if data.has_key("size") {
                size = (
                    data["size"]["w"].as_f32().expect("Parse f32: size w"),
                    data["size"]["h"].as_f32().expect("Parse f32: size h")
                );
            } else { return reserr("Object JSON doesn't habe key `size`"); }
            let object = Object::new()
                .name(name)
                .otype(otype)
                .pos(pos.0, pos.1)
                .size(size.0, size.1);
            objects.push(object);
        }
    } else {
        return reserr("JSON file doesn't have key `tiles`");
    }

    Ok((tilesets, tiles, objects))
}

fn reserr<T: ToString, U>(msg: T) -> Result<U, GameError> {
    Err(GameError::ResourceLoadError(msg.to_string()))
}
