use std::ops::{Index, IndexMut};

pub struct Config {
    pub border_width: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            border_width: 5,
        }
    }
}

impl Index<&'_ str> for Config {
    type Output = u16;
    fn index(&self, value: &str) -> &u16 {
        match value {
            "border_width" => &self.border_width,
            _ => &0,
        }
    }
}

impl IndexMut<&'_ str> for Config {
    fn index_mut(&mut self, value: &str) -> &mut u16 {
        match value {
            "border_width" => &mut self.border_width,
            _ => panic!("Unknown field"),
        }
    }
}
