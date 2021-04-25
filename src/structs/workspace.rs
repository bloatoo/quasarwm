use super::window::Window;

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
    
    pub fn remove_window(&mut self, identifier: u32) {
        self.windows.retain(|win| win.identifier != identifier)
    }
}
