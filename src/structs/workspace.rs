use xcb::Connection;

use super::window::Window;

pub enum Layout {
    Spiral,
}

pub struct Workspace {
    pub windows: Vec<Window>,
    pub name: String,
}

impl Workspace {
    pub fn has_window(&self, identifier: u32) -> bool {
        self.windows
            .iter()
            .find(|win| win.identifier == identifier).is_some()
    }

    pub fn resize(&mut self, layout: Layout, width: u32, height: u32, conn: &Connection) {
        let count = self.windows.len();

        match layout {
            Layout::Spiral => {
                let geos = Window::geometry(
                    count,
                    0,
                    5,
                    5,
                    width - 20,
                    height - 20,
                    if count % 2 == 0 {
                        false
                    } else {
                        true
                    }
                );

                for (i, window) in self.windows.iter_mut().enumerate() {
                    let geo = geos[i];

                    window.resize(geo);

                    xcb::configure_window(conn, window.identifier, &[
                        (xcb::CONFIG_WINDOW_X as u16, geo.x),
                        (xcb::CONFIG_WINDOW_Y as u16, geo.y),
                        (xcb::CONFIG_WINDOW_WIDTH as u16, geo.width),
                        (xcb::CONFIG_WINDOW_HEIGHT as u16, geo.height),
                        (xcb::CONFIG_WINDOW_BORDER_WIDTH as u16, 5),
                    ]);
                }
            }
        }
    }
    
    pub fn remove_window(&mut self, identifier: u32) {
        self.windows.retain(|win| win.identifier != identifier)
    }
}
