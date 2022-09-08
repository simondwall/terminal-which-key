#![allow(unused)]
use std::io::stdout;

use crossterm::{
    cursor::{MoveLeft, MoveTo, MoveUp, SavePosition, RestorePosition},
    terminal::size, queue, execute, style::Print,
};

use crate::keybinding::KeyBinding;

pub struct Window {
    keybindings: Vec<KeyBinding>,
}

impl Window {
    pub fn new(keybindings: Vec<KeyBinding>) -> Self {
        Window { keybindings }
    }

    pub fn draw_border(&self, height: u16, width: u16) {
        let (cols, rows) = size().unwrap();
        execute!(stdout(), SavePosition, MoveTo(cols/2, rows/2), Print("+"), RestorePosition).unwrap();
    }

    fn get_grid_size(&self) -> (u8, u8) {
        // keep ratio of screen as well as possible
        (0, 0)
    }

    fn get_max_length(&self) -> u8 {
        self.keybindings
            .iter()
            .map(|keybinding| keybinding.length())
            .max()
            .unwrap()
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        // redraw replaced_region
        // todo!()
    }
}
