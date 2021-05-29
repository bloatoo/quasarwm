use std::{collections::HashMap, process::Command, sync::mpsc::{self, Sender}, thread};

use crate::response_type;
use super::window::{Window, Rect};
use super::workspace::{Layout, Workspace};
use std::os::unix::net::{UnixListener, UnixStream}; use std::io::{BufRead, BufReader};
use super::config::Config;
use crate::utils;

pub struct Quasar {
    pub workspaces: Vec<Workspace>, pub current_workspace: usize, pub conn: xcb::Connection, pub config: Config,
    pub commands: HashMap<u8, fn() -> ()>,
    pub actions: HashMap<u8, Action>,
}

pub enum Action {
    CloseWindow,
    CycleWindowUp,
    CycleWindowDown,
    Exit,
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
        let mut actions: HashMap<u8, Action> = HashMap::new();

        //commands.insert(36, || { Command::new("alacritty").arg("-e").arg("zsh").spawn().unwrap(); });
        commands.insert(36, || { Command::new("alacritty").spawn().unwrap(); });
        commands.insert(33, || { Command::new("dmenu_run").spawn().unwrap(); });

        actions.insert(54, Action::CloseWindow);
        actions.insert(45, Action::CycleWindowUp);
        actions.insert(44, Action::CycleWindowDown);
        actions.insert(24, Action::Exit);

        let mod_mask = xcb::MOD_MASK_4;

        for key in actions.iter().map(|elem| elem.0) {
            xcb::grab_key(&conn, false, screen.root(), mod_mask as u16, *key, xcb::GRAB_MODE_ASYNC as u8, xcb::GRAB_MODE_ASYNC as u8);
        }

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
                gap: 10,
                windows: Vec::new(),
                focused_window: 0,
            })
        }

        Self {
            conn,
            commands,
            actions,
            config: Config::default(),
            current_workspace: 0,
            workspaces,
        }
    }

    fn add_window(&mut self, window: Window) {
        match self.workspaces.get_mut(self.current_workspace) {
            Some(workspace) => {
                if workspace.windows.iter().find(|win| win.identifier == window.identifier).is_none() {
                    workspace.windows.push(window);
                    workspace.focus_down();
                }
            }

            None => (),
        }
    }

    fn del_window(&mut self, window: u32) {
        for workspace in &mut self.workspaces {
            if workspace.has_window(window) {
                workspace.remove_window(window);
                workspace.focus_up();
            }
        }
    }

    pub fn run(&mut self) {
        let (sender, receiver) = mpsc::channel::<String>();

        thread::spawn(move || {
            let path_str = "/tmp/quasarwm.sock";

            if std::path::Path::new(path_str).exists() {
                std::fs::remove_file(path_str).unwrap();
            }

            let listener = UnixListener::bind(path_str).unwrap();

            fn handle_conn(stream: UnixStream, sender: Sender<String>) {
                let stream = BufReader::new(stream);

                for line in stream.lines() {
                    sender.send(line.unwrap()).unwrap();
                }
            }

            for stream in listener.incoming() {
                let sender = sender.clone();

                match stream {
                    Ok(stream) => {
                        thread::spawn(move || handle_conn(stream, sender));
                    },
                     
                    Err(err) => {
                        println!("Error: {}", err);
                        break;
                    }
                }
            }
        });

        loop {
            let event = self.conn.poll_for_event();

            if let Ok(msg) = receiver.try_recv() {
                utils::parse_command(msg, &mut self.config);
            }

            match event {
                Some(event) => {
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
                            
                            } else if let Some(action) = self.actions.get(&keycode){ 
                                let workspace = self.workspaces.get_mut(self.current_workspace).unwrap();

                                match action {
                                    Action::CloseWindow => workspace.close_focused_window(&self.conn),
                                    Action::CycleWindowUp => workspace.focus_up(),
                                    Action::CycleWindowDown => workspace.focus_down(),
                                    Action::Exit => break,
                                }
                                
                                let screen = self.conn.get_setup().roots().nth(0).unwrap();
                                
                                let rect = Rect::new(0, 0,  screen.width_in_pixels() as u32, screen.height_in_pixels() as u32);
                                workspace.resize(Layout::Quasar, rect.width, rect.height, self.config.border_width as u32, &self.conn);
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
                            
                            let screen = self.conn.get_setup().roots().nth(0).unwrap();
                            
                            let rect = Rect::new(0, 0,  screen.width_in_pixels() as u32, screen.height_in_pixels() as u32);
                            
                            let window = Window::new(ev.window(), rect);
                            
                            self.add_window(window);
                            
                            let workspace = self.workspaces.get_mut(self.current_workspace).unwrap();
                            workspace.resize(Layout::Quasar, rect.width, rect.height, self.config.border_width as u32, &self.conn);
                        
                            xcb::change_window_attributes(&self.conn, ev.window(), &[
                                (xcb::CW_BORDER_PIXEL, if workspace.windows.len() - 1 == workspace.focused_window { 0xab4642 } else { 0x282828 }),
                            ]);
                        }
                
                        DestroyNotify => {
                            let ev: &xcb::DestroyNotifyEvent = unsafe {
                                xcb::cast_event(&event)
                            };
                      
                            let screen = self.conn.get_setup().roots().nth(0).unwrap();
                            let height = screen.height_in_pixels() as u32;
                            let width = screen.width_in_pixels() as u32;
                            
                            self.del_window(ev.window());
                            
                            let workspace = self.workspaces.get_mut(self.current_workspace).unwrap();
                            workspace.resize(Layout::Quasar, width, height, self.config.border_width as u32, &self.conn);
                            
                        }
                        
                        _ => ()
                    }
                    
                    self.conn.flush();
                }

                None => ()
            }
        }   
    }   

    pub fn change_workspace(&mut self, workspace_idx: usize) {
        if self.current_workspace != workspace_idx {
            let workspace = self.workspaces.get(self.current_workspace).unwrap();

            for win in workspace.windows.clone() {
                xcb::unmap_window(&self.conn, win.identifier);
            }

            let workspace_new = self.workspaces.get_mut(workspace_idx).unwrap();
            
            self.current_workspace = workspace_idx;

            let screen = self.conn.get_setup().roots().nth(0).unwrap();
             
            for win in workspace_new.windows.clone() {
                xcb::map_window(&self.conn, win.identifier);
            }

            workspace_new.resize(Layout::Quasar, screen.width_in_pixels() as u32, screen.height_in_pixels() as u32, self.config.border_width as u32, &self.conn);
        }
    }
}
