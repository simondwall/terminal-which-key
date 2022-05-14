#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod config;

use config::{
    keybindings::{self, Commands, Keybinding},
    styles::{CrosstermColor, Styles},
};
use crossterm::{
    cursor::{MoveTo, MoveToNextLine},
    event::{read, Event::Key, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::{Attribute, Attributes, Color, Print, PrintStyledContent, StyledContent, Stylize},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};
use std::io::{stdout, Write};

fn main() -> Result<()> {
    // Initialize
    let (styles, keybindings) = load_config();
    queue!(stdout(), EnterAlternateScreen)?;

    // Run
    create_popup(&keybindings, &styles)?;
    // let labels = create_labels(&keybindings, &styles);
    // for label in labels {
    //     queue!(stdout(), Print(label), MoveToNextLine(1))?;
    // }
    stdout().flush()?;
    read()?;

    // Destruct
    execute!(stdout(), LeaveAlternateScreen)
}

fn load_config() -> (Styles, Vec<Keybinding>) {
    let styles_file =
        std::fs::File::open("/Users/simondawall/.config/terminal-which-key/styles.yaml")
            .expect("Styles file could not be opened");
    let keybindings_file =
        std::fs::File::open("/Users/simondawall/.config/terminal-which-key/keybindings.yaml")
            .expect("Keybindings file could not be opened");

    let styles: Styles =
        serde_yaml::from_reader(styles_file).expect("Styles file is not of currect format");
    let keybindings: Vec<Keybinding> = serde_yaml::from_reader(keybindings_file)
        .expect("Keybindings file is not of correct format");

    (styles, keybindings)
}

fn create_labels(keybindings: &[Keybinding], styles: &Styles) -> Vec<String> {
    keybindings
        .iter()
        .map(|binding| {
            format!(
                "{key} {arrow} {name} {menu}",
                key = binding
                    .key
                    .with(styles.colors.popup_foreground.to_color())
                    .on(styles.colors.popup_background.to_color()),
                arrow = styles
                    .icons
                    .arrow
                    .as_str()
                    .with(styles.colors.arrow_foreground.to_color())
                    .on(styles.colors.arrow_background.to_color()),
                name = binding
                    .name
                    .as_str()
                    .with(styles.colors.popup_foreground.to_color())
                    .on(styles.colors.popup_background.to_color()),
                menu = match binding.commands {
                    Commands::Menu(_) => &styles.icons.menu,
                    _ => "",
                }
                .with(styles.colors.menu_foreground.to_color())
                .on(styles.colors.menu_background.to_color())
            )
        })
        .collect()
}

fn create_popup(keybindings: &[Keybinding], style: &Styles) -> Result<()> {
    let mut stdout = stdout();
    let size = crossterm::terminal::size()?;
    let mid_x = i32::from(size.0 / 2);
    let mid_y = i32::from(size.1 / 2);

    let number: i32 = keybindings.len().try_into().unwrap();

    let columns: i32 = if number <= 4 { number } else { 5 };

    let rows: i32 = number / columns + if number % columns >= 1 { 0 } else { 1 };

    let labels = create_labels(keybindings, style);
    let column_size: i32 = keybindings
        .iter()
        .map(|binding| style.icons.arrow.len() + style.icons.menu.len() + binding.name.len() + 4)
        .max()
        .unwrap()
        .try_into()
        .unwrap();

    let (x_size, y_size) = (columns * column_size + columns * 2, rows + 2);
    let uneven_x = x_size % 2;
    let uneven_y = y_size % 2;

    let top = style
        .border
        .top_edge
        .as_str()
        .with(style.colors.border_foreground.to_color())
        .on(style.colors.border_background.to_color());
    let bottom = style
        .border
        .bottom_edge
        .as_str()
        .with(style.colors.border_foreground.to_color())
        .on(style.colors.border_background.to_color());
    let left = style
        .border
        .left_edge
        .as_str()
        .with(style.colors.border_foreground.to_color())
        .on(style.colors.border_background.to_color());
    let right = style
        .border
        .right_edge
        .as_str()
        .with(style.colors.border_foreground.to_color())
        .on(style.colors.border_background.to_color());
    let top_right = style
        .border
        .top_right_corner
        .as_str()
        .with(style.colors.border_foreground.to_color())
        .on(style.colors.border_background.to_color());
    let top_left = style
        .border
        .top_left_corner
        .as_str()
        .with(style.colors.border_foreground.to_color())
        .on(style.colors.border_background.to_color());
    let bottom_right = style
        .border
        .bottom_right_corner
        .as_str()
        .with(style.colors.border_foreground.to_color())
        .on(style.colors.border_background.to_color());
    let bottom_left = style
        .border
        .bottom_left_corner
        .as_str()
        .with(style.colors.border_foreground.to_color())
        .on(style.colors.border_background.to_color());

    // DEBUG
    println!("Middle: x: {:?}, y: {:?}", mid_x, mid_y);
    println!("Number of Bindings: {:?}", number);
    println!("Columns Size: {:?}", column_size);
    println!("Labels: x: {:?}, y: {:?}", columns, rows);
    println!("Size: x: {:?}, y: {:?}", x_size, y_size);

    // Draw Top
    queue!(
        stdout,
        MoveTo(
            (mid_x - x_size / 2).try_into().unwrap_or(0),
            (mid_y - y_size / 2).try_into().unwrap_or(0)
        ),
        PrintStyledContent(top_left),
    )?;
    for _ in 1..x_size - uneven_x {
        queue!(stdout, PrintStyledContent(top),)?;
    }
    queue!(stdout, PrintStyledContent(top_right),)?;

    // Draw Center
    for y in 1..y_size - uneven_y {
        queue!(
            stdout,
            MoveTo(
                (mid_x - x_size / 2).try_into().unwrap(),
                (mid_y - (y - y_size / 2)).try_into().unwrap()
            ),
            PrintStyledContent(left),
        )?;
    }
    for y in 1..y_size - uneven_y {
        queue!(
            stdout,
            MoveTo(
                (mid_x - x_size / 2 + 1).try_into().unwrap(),
                (mid_y - (y - y_size / 2)).try_into().unwrap()
            )
        )?;
        for _ in 1..x_size {
            queue!(
                stdout,
                PrintStyledContent(
                    " ".with(style.colors.popup_foreground.to_color())
                        .on(style.colors.popup_background.to_color())
                ),
            )?;
        }
    }
    for y in 1..y_size - uneven_y {
        queue!(
            stdout,
            MoveTo(
                (mid_x + x_size / 2).try_into().unwrap(),
                (mid_y - (y - y_size / 2)).try_into().unwrap()
            ),
            PrintStyledContent(right),
        )?;
    }

    // Labels
    for c in 0..columns {
        for r in 1..rows {
            queue!(
                stdout,
                MoveTo(
                    (mid_x - x_size / 2 + c * (column_size + 2) + 3)
                        .try_into()
                        .unwrap(),
                    (mid_y - y_size / 2 + r).try_into().unwrap()
                ),
                Print(&labels[(c + (r - 1) * columns) as usize])
            )?;
        }
    }

    // Draw Bottom
    queue!(
        stdout,
        MoveTo(
            (mid_x - x_size / 2).try_into().unwrap(),
            (mid_y + y_size / 2).try_into().unwrap()
        ),
        PrintStyledContent(bottom_left),
    )?;
    for _ in 1..x_size - uneven_x {
        queue!(stdout, PrintStyledContent(bottom),)?;
    }
    queue!(stdout, PrintStyledContent(bottom_right),)
}

fn stuff() {
    // enable_raw_mode()?;
    // loop {
    //     match read() {
    //         // Leader Key
    //         Ok(Key(KeyEvent {
    //             code: KeyCode::Char(' '),
    //             modifiers: KeyModifiers::NONE,
    //         })) => execute!(stdout(),MoveTo(0,0), Print("pressed"))?,
    //
    //         // Quitting
    //         Ok(Key(KeyEvent {
    //             code: KeyCode::Char('q'),
    //             modifiers: KeyModifiers::CONTROL,
    //         })) => break,
    //
    //         // All other Events
    //         event => {
    //             execute!(stdout(), MoveTo(0, 0), Print(format!("{:?}", event)))?;
    //         }
    //     };
    // }
    // disable_raw_mode()
}
