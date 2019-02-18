use ggez::nalgebra::Point2;

use crate::Size;

#[derive(Debug)]
pub struct Object {
    pub name:  String,
    pub otype: String,
    pub pos:   Point2<f32>,
    pub size:  Size<f32>,
}

impl Object {
    pub fn new() -> Self {
        Self {
            name:  String::from(""),
            otype: String::from(""),
            pos:   Point2::new(0.0, 0.0),
            size:  Size::new(0.0, 0.0),
        }
    }

    pub fn name<T: ToString>(mut self, name: T) -> Self {
        self.name = name.to_string();
        self
    }
    pub fn otype<T: ToString>(mut self, otype: T) -> Self {
        self.otype = otype.to_string();
        self
    }
    pub fn pos<T: Into<f32> + std::fmt::Debug>(mut self, x: T, y: T) -> Self {
        self.pos = Point2::new(x.into(), y.into());
        self
    }
    pub fn size<T: Into<f32>>(mut self, w: T, h: T) -> Self {
        self.size = Size::new(w.into(), h.into());
        self
    }
}
