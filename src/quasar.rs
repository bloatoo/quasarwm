use std::{
    collections::HashMap,
    process::Command,
};

use crate::response_type;

pub struct Workspace {
    pub windows: Vec<u32>,
    pub name: String,
}
pub struct Quasar {
    pub workspaces: Vec<Workspace>,
    pub current_workspace: usize,
    pub conn: xcb::Connection,
    pub commands: HashMap<u8, fn() -> ()>,
}

impl Quasar {
    pub fn default() -> Self {
        let (conn, _) = xcb::Connection::connect(None).unwrap();
        let setup = conn.get_setup();
        let screen = setup.roots().nth(0).unwrap();
        conn.flush();

        let workspace_names = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"];

        let mut workspaces: Vec<Workspace> = Vec::new();

        let mut commands: HashMap<u8, fn() -> ()> = HashMap::new();

        commands.insert(28, || { Command::new("alacritty").spawn().unwrap(); });
        commands.insert(33, || { Command::new("dmenu_run").spawn().unwrap(); });

        let mod_mask = xcb::MOD_MASK_4;

        for key in commands.iter().map(|elem| elem.0) {
            xcb::grab_key(&conn, false, screen.root(), mod_mask as u16, *key, xcb::GRAB_MODE_ASYNC as u8, xcb::GRAB_MODE_ASYNC as u8);
        }

        for key in 10..=20 {
            xcb::grab_key(&conn, false, screen.root(), mod_mask as u16, key, xcb::GRAB_MODE_ASYNC as u8, xcb::GRAB_MODE_ASYNC as u8);
        }

        xcb::change_window_attributes(&conn, screen.root(), &[(xcb::CW_EVENT_MASK, xcb::EVENT_MASK_SUBSTRUCTURE_NOTIFY)]);

        for name in workspace_names {
            let name = name.to_string();

            workspaces.push(Workspace {
                name,
                windows: Vec::new(),
            })
        }

        Self {
            conn,
            commands,
            current_workspace: 0,
            workspaces,
        }
    }

    fn add_window(&mut self, window: u32) {
        match self.workspaces.get_mut(self.current_workspace) {
            Some(workspace) => {
                workspace.windows.push(window);
            }

            None => (),
        }
    }

    fn del_window(&mut self, window: u32) {
        for workspace in &mut self.workspaces {
            if workspace.windows.contains(&window) {
                workspace.windows.retain(|win| win != &window);
            }
        }
    }


    pub fn run(&mut self) {
        loop {
            let event = self.conn.wait_for_event().unwrap();
            let r = event.response_type();

            use response_type::ResponseType::{self, *};

            match ResponseType::from(r) {
                KeyPress => {
                    let ev: &xcb::KeyPressEvent = unsafe {
                        xcb::cast_event(&event)
                    };
                    
                    let keycode = ev.detail();
                    
                    if let Some(command) = self.commands.get(&keycode) {
                        command();
                    } else {
                        match keycode {
                            10..=20 => self.change_workspace(keycode as usize - 10),
                          _ => ()
                        }
                    }
                }

                MapNotify => {
                    let ev: &xcb::MapNotifyEvent = unsafe {
                        xcb::cast_event(&event)
                    };
                    self.add_window(ev.window());
                    let screen = self.conn.get_setup().roots().nth(0).unwrap();

                    xcb::configure_window(&self.conn, ev.window(), &[
                        (xcb::CONFIG_WINDOW_X as u16, 0),
                        (xcb::CONFIG_WINDOW_Y as u16, 0),
                        (xcb::CONFIG_WINDOW_WIDTH as u16, screen.width_in_pixels() as u32),
                        (xcb::CONFIG_WINDOW_HEIGHT as u16, screen.height_in_pixels() as u32),
                    ]);
                }
                
                DestroyNotify => {
                    let ev: &xcb::DestroyNotifyEvent = unsafe {
                        xcb::cast_event(&event)
                    };
                     
                    self.del_window(ev.window());
                }

                _ => ()
            }

            self.conn.flush();
        }   
    }   

    pub fn change_workspace(&mut self, workspace_idx: usize) {
        if self.current_workspace != workspace_idx {
            let workspace = self.workspaces.get(self.current_workspace).unwrap();

            for win in workspace.windows.clone() {
                xcb::unmap_window(&self.conn, win);
            }

            let workspace_new = self.workspaces.get(workspace_idx).unwrap();
            
            self.current_workspace = workspace_idx;
             
            for win in workspace_new.windows.clone() {
                xcb::map_window(&self.conn, win);
            }
        }
    }
}
