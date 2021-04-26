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
    
    pub fn resize(&mut self, area: Rect) {
        self.area = area;
    }

    pub fn geometry(window_count: usize, idx: usize, x: u32, y: u32, width: u32, height: u32, vertical: bool) -> Vec<Rect> {
        match window_count {
            0 => vec![],
            1 => vec![Rect::new(x, y, width, height)],
            _ => {
                match vertical {
                    true => {
                        let mut vec = vec![Rect::new(x, y, width, height / 2)];
                        vec.append(&mut Window::geometry(window_count - 1, idx + 1, x, y + height / 2 ,width, height / 2, false));
                        vec
                    },
    
                    false => {
                        let mut vec = vec![Rect::new(x, y, width / 2, height)];
                        vec.append(&mut Window::geometry(window_count - 1, idx + 1, x + width / 2, y, width / 2, height, true));
                        vec
                    }
                }
            }
        }

    }
}
