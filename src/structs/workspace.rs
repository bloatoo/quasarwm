use xcb::Connection;

use super::window::Window;

pub enum Layout {
    Quasar,
}

pub struct Workspace {
    pub windows: Vec<Window>,
    pub name: String,
    pub gap: u32,
    pub focused_window: usize,
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
            Layout::Quasar => {
                let geos = Window::geometry(
                    count,
                    0,
                    self.gap,
                    self.gap,
                    width,
                    height,
                    1
                );

                for (i, window) in self.windows.iter_mut().enumerate() {
                    let geo = geos[i];

                    window.resize(geo);

                    xcb::configure_window(conn, window.identifier, &[
                        (xcb::CONFIG_WINDOW_X as u16, geo.x),
                        (xcb::CONFIG_WINDOW_Y as u16, geo.y),
                        (xcb::CONFIG_WINDOW_WIDTH as u16, geo.width - self.gap * 2 - window.border.width as u32 * 2),
                        (xcb::CONFIG_WINDOW_HEIGHT as u16, geo.height - self.gap * 2 - window.border.width as u32 * 2),
                        (xcb::CONFIG_WINDOW_BORDER_WIDTH as u16, window.border.width as u32),
                    ]);

                    xcb::change_window_attributes(&conn, window.identifier, &[
                        (xcb::CW_BORDER_PIXEL, if i == self.focused_window { 0xab4642 } else { 0x282828 }),
                    ]);

                    if i == self.focused_window {
                        xcb::set_input_focus(&conn, xcb::INPUT_FOCUS_PARENT as u8, window.identifier, 0);
                    }
                }
            }
        }
    }

    pub fn close_focused_window(&mut self, conn: &Connection) {
        xcb::destroy_window(conn, self.windows.get(self.focused_window).unwrap().identifier as u32);

        /*if self.windows.len() > 0 {
            self.focus_up();
        }*/

        self.remove_window(self.focused_window as u32);
    }

    pub fn focus_down(&mut self) {//conn: &Connection, idx: usize, width: u32, height: u32) {
        let idx = self.focused_window;

        if idx + 1 >= self.windows.len() {
            self.focused_window = 0;
            return;
        }

        self.focused_window += 1;
    }

    pub fn focus_up(&mut self) {
        let idx = self.focused_window;

        if idx == 0 {
            self.focused_window = self.windows.len() - 1;
            return;
        }

        self.focused_window -= 1;
    }
    
    pub fn remove_window(&mut self, identifier: u32) {
        self.windows.retain(|win| win.identifier != identifier)
    }
}
