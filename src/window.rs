#![allow(unused)]
use std::io::{stdout, Write};

use crossterm::{
    cursor::{MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp, RestorePosition, SavePosition},
    execute, queue,
    style::Print,
    terminal::size,
};

use crate::config::FormatConfig;

pub struct Window {
}

impl Window {
    pub fn new(config: crate::config::Config) -> Self {
        Self {}
    }

    pub fn draw_border(&self, height: u16, width: u16) {
        let (cols, rows) = size().unwrap();
        let mut out = stdout();
        queue!(
            out,
            SavePosition,
            MoveTo(cols / 2, rows / 2),
            MoveUp(height / 2),
            MoveLeft(width / 2),
            Print("┏"),
        )
        .unwrap();
        for _ in 2..width {
            queue!(out, Print("━")).unwrap();
        }
        queue!(out, Print("┓")).unwrap();
        for _ in 2..height {
            queue!(out, MoveDown(1), MoveLeft(1), Print("┃"));
        }
        queue!(out, MoveDown(1), MoveLeft(1), Print("┛")).unwrap();
        for _ in 2..width {
            queue!(out, MoveLeft(2), Print("━")).unwrap();
        }
        queue!(out, MoveLeft(2), Print("┗")).unwrap();
        for _ in 2..height {
            queue!(out, MoveUp(1), MoveLeft(1), Print("┃"));
        }
        queue!(out, RestorePosition).unwrap();
        out.flush();
    }

    fn get_grid_size(&self) -> (u8, u8) {
        // keep ratio of screen as well as possible
        (0, 0)
    }

    fn get_max_length(&self) -> u8 {
        0
    }
}
