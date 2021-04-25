#[derive(Clone, Copy)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy)]
pub struct Window {
    pub area: Rect,
    pub identifier: u32,
}

impl Rect {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            width: width.into(),
            height: height.into(),
        }
    }
}

impl Window {
    pub fn new(identifier: u32, area: Rect) -> Self {
        Self {
            identifier,
            area,
        }
    }
}
