#![allow(unused)]
use std::{
    collections::HashMap,
    io::{stdout, Write},
};

use crossterm::{
    cursor::{MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp, RestorePosition, SavePosition},
    execute, queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::size,
};

use crate::{
    config::{Child, MenuConfig},
    keys::Display,
};

pub struct Window {
    labels: Vec<String>,
    cols: u16,
    rows: u16,
}

impl<'a> Window {
    pub fn new(current_menu: &'a MenuConfig) -> Self {
        let labels: Vec<String> = current_menu
            .children
            .iter()
            .map(|child| match child {
                Child::Menu(m) => {
                    format!("{key} -> {name}[+]", key = m.key.display(), name = m.name)
                }
                Child::Action(a) => {
                    format!("{key} -> {name}", key = a.key.display(), name = a.name)
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
                    MoveTo(
                        self.cols / 2 + col * length - cols * length / 2,
                        self.rows / 2 + row - rows / 2
                    ),
                    Print(format!(
                        " {} ",
                        self.labels
                            .get((col % cols + row * cols) as usize)
                            .unwrap_or(&"".to_string())
                    ))
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
            SetForegroundColor(Color::Blue),
            MoveTo(cols / 2, rows / 2),
            MoveUp(height / 2),
            MoveLeft(width / 2),
        )
        .unwrap();

        for y in 1..=height {
            for x in 1..=width {
                if x == 1 {
                    if y == 1 {
                        queue!(out, Print("┏")).unwrap();
                    } else if y == height {
                        queue!(out, Print("┗")).unwrap();
                    } else {
                        queue!(out, Print("┃")).unwrap();
                    }
                } else if x == width {
                    if y == 1 {
                        queue!(out, Print("┓"), MoveDown(1), MoveLeft(width)).unwrap();
                    } else if y == height {
                        queue!(out, Print("┛")).unwrap();
                    } else {
                        queue!(out, Print("┃"), MoveDown(1), MoveLeft(width)).unwrap();
                    }
                } else {
                    if y == 1 || y == height {
                        queue!(out, Print("━")).unwrap();
                    } else {
                        queue!(out, Print(" ")).unwrap();
                    }
                }
            }
        }
        queue!(out, ResetColor, RestorePosition).unwrap();
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
