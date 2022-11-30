#![allow(unused)]
use std::{
    collections::HashMap,
    io::{stdout, Write},
};

use crossterm::{
    cursor::{MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp, RestorePosition, SavePosition},
    execute, queue,
    style::Print,
    terminal::size,
};

use crate::config::{Child, MenuConfig};

pub struct Window {
    labels: Vec<String>,
    cols: u16,
    rows: u16
}

impl<'a> Window {
    pub fn new(current_menu: &'a MenuConfig) -> Self {
        let labels: Vec<String> = current_menu
            .children
            .iter()
            .map(|child| match child {
                Child::Menu(m) => {
                    format!("{key} -> {name}[+]", key = m.key.to_string(), name = m.name)
                }
                Child::Action(a) => {
                    format!("{key} -> {name}", key = a.key.to_string(), name = a.name)
                }
            })
            .collect();

        let (cols, rows) = crossterm::terminal::size().unwrap();

        Self { labels, cols, rows }
    }

    pub fn draw(&self) {
        let length = self.get_max_length() + 2;
        let (cols, rows) = self.get_grid_size();

        self.draw_border(rows + 2, cols * length + 4);
        self.draw_labels(length, cols, rows);
    }

    fn draw_labels(&self, length: u16, cols: u16, rows: u16) {
        let mut out = stdout();
        queue!(out, SavePosition).unwrap();
        for col in 0..cols {
            for row in 0..rows {
                queue!(
                    out,
                    MoveTo(self.cols/2 + col * length - cols * length / 2, self.rows / 2 + row - rows / 2),
                    Print(format!(" {} ", self.labels.get((col % cols + row * cols) as usize).unwrap_or(&"".to_string())))
                )
                .unwrap();
            }
        }
        queue!(out, RestorePosition).unwrap();
        out.flush();
    }

    fn draw_border(&self, height: u16, width: u16) {
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

    fn get_grid_size(&self) -> (u16, u16) {
        let amount = self.labels.len();
        let cols = (amount as f32).sqrt().floor();
        let rows = (amount as f32 / cols).ceil();

        (rows as u16, cols as u16)
    }

    fn get_max_length(&self) -> u16 {
        self.labels
            .iter()
            .map(|label| label.chars().count())
            .max()
            .unwrap() as u16
    }
}
