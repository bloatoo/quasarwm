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
    pub border: Border,
}

#[derive(Clone, Copy)]
pub struct Border {
    pub width: u16,
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

impl From<u16> for Border {
    fn from(data: u16) -> Self {
        Self {
            width: data.into(),
        }
    }
}

impl Window {
    pub fn new(identifier: u32, area: Rect) -> Self {
        Self {
            identifier,
            area,
            border: Border::from(5),
        }
    }
    
    pub fn resize(&mut self, area: Rect) {
        self.area = area;
    }

    pub fn geometry(window_count: usize, idx: usize, x: u32, y: u32, width: u32, height: u32, vertical: usize) -> Vec<Rect> {
        match window_count {
            0 => vec![],
            1 => vec![Rect::new(x, y, width, height)],
            _ => {
                if idx % 2 == vertical {
                        let mut vec = vec![Rect::new(x, y, width, height / 2)];
                        vec.append(&mut Window::geometry(window_count - 1, idx + 1, x, y + height / 2 ,width, height / 2, 0));
                        vec
                } else {
                        let mut vec = vec![Rect::new(x, y, width / 2, height)];
                        vec.append(&mut Window::geometry(window_count - 1, idx + 1, x + width / 2, y, width / 2, height, 1));
                        vec
                }
            }
        }

    }
}
