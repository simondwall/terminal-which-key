mod config;

use config::{
    keybindings::{Commands, Keybinding},
    styles::{CrosstermColor, Styles},
};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{read, Event::Key, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::{Print, Stylize},
    terminal::{
        disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    Result,
};
use std::{
    cmp::max,
    io::{stdout, Write},
    process::Command,
};

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    let args_str: Vec<&str> = args.iter().map(|arg| &arg[..]).collect();

    let (style, keybindings) = load_config();

    match args_str[..] {
        [_, "width"] => println!("{}", get_size(&keybindings, &style).0),
        [_, "height"] => println!("{}", get_size(&keybindings, &style).1),
        [_, "run"] => run(&keybindings, &style)?,
        _ => {}
    }

    Ok(())
}

fn run(keybindings: &[Keybinding], style: &Styles) -> Result<()> {
    // Initialize
    enable_raw_mode()?;
    queue!(stdout(), EnterAlternateScreen, Hide)?;

    // Run
    let mut current_keybindings = keybindings;
    let mut pressed_keys: Vec<char> = Vec::new();
    loop {
        queue!(stdout(), Clear(ClearType::All))?;
        draw(current_keybindings, style)?;
        stdout().flush()?;
        match read()? {
            Key(KeyEvent {
                code: KeyCode::Char(key),
                modifiers: KeyModifiers::NONE,
            }) => {
                match run_command(current_keybindings, &key) {
                    Some(new_keybindings) => current_keybindings = new_keybindings,
                    None => break,
                };
                pressed_keys.push(key)
            }
            Key(KeyEvent {
                code: KeyCode::Backspace,
                modifiers: KeyModifiers::NONE
            }) => {
                if pressed_keys.is_empty() {
                    break;
                }
                pressed_keys.pop();
                let mut new_keybindings = keybindings;
                for pressed_key in &pressed_keys {
                    match run_command(new_keybindings, pressed_key) {
                        Some(b) => new_keybindings = b,
                        None => break,
                    }
                }
                current_keybindings = new_keybindings;
            },
            Key(KeyEvent {
                code: KeyCode::Esc,
                modifiers: _,
            }) => break,
            _ => (),
        };
    }

    // Destruct
    disable_raw_mode()?;
    execute!(stdout(), Show, LeaveAlternateScreen)
}

fn run_command<'a>(keybindings: &'a [Keybinding], key: &char) -> Option<&'a [Keybinding]> {
    for keybinding in keybindings {
        if &keybinding.key == key {
            return match &keybinding.commands {
                Commands::Menu(new_keybindings) => Some(new_keybindings),
                Commands::Command(command) => {
                    Command::new("sh").arg("-c").arg(command).output().unwrap();
                    None
                }
            };
        }
    }
    None
}

fn get_local_column_size(keybindings: &[Keybinding], style: &Styles) -> u16 {
    keybindings
        .iter()
        .map(|binding| style.icons.arrow.len() + style.icons.menu.len() + binding.name.len() + 4) // key char + 3 * spaces
        .max()
        .unwrap_or(0)
        .try_into()
        .unwrap_or(0)
}

fn load_config() -> (Styles, Vec<Keybinding>) {
    let style_file = std::fs::File::open(format!(
        "{}/.config/terminal-which-key/styles.yaml",
        std::env::var("HOME").expect("HOME Environment Variable is not set")
    ))
    .expect("Styles file could not be opened");
    let keybindings_file = std::fs::File::open(format!(
        "{}/.config/terminal-which-key/keybindings.yaml",
        std::env::var("HOME").expect("HOME Environment Variable is not set")
    ))
    .expect("Keybindings file could not be opened");

    let style: Styles =
        serde_yaml::from_reader(style_file).expect("Styles file is not of currect format");
    let keybindings: Vec<Keybinding> = serde_yaml::from_reader(keybindings_file)
        .expect("Keybindings file is not of correct format");

    (style, keybindings)
}

fn create_labels(keybindings: &[Keybinding], style: &Styles) -> Vec<String> {
    keybindings
        .iter()
        .map(|binding| {
            format!(
                "{key} {arrow} {name} {menu}",
                key = binding
                    .key
                    .with(style.colors.key_foreground.to_color())
                    .on(style.colors.key_background.to_color()),
                arrow = style
                    .icons
                    .arrow
                    .as_str()
                    .with(style.colors.arrow_foreground.to_color())
                    .on(style.colors.arrow_background.to_color()),
                name = binding
                    .name
                    .as_str()
                    .with(style.colors.name_foreground.to_color())
                    .on(style.colors.name_background.to_color()),
                menu = match binding.commands {
                    Commands::Menu(_) => &style.icons.menu,
                    _ => "",
                }
                .with(style.colors.menu_foreground.to_color())
                .on(style.colors.menu_background.to_color())
            )
        })
        .collect()
}

fn draw(keybindings: &[Keybinding], style: &Styles) -> Result<()> {
    let width = size().unwrap().0;
    let height = size().unwrap().1;
    let labels = create_labels(keybindings, style);
    let column_size = get_local_column_size(keybindings, style);

    let label_amount: u16 = labels.len() as u16;

    let columns = (label_amount as f64).sqrt().ceil() as u16;
    let rows = label_amount / columns
        + if columns + label_amount / columns < label_amount {
            1
        } else {
            0
        }
        - if label_amount % columns > 0 { 0 } else { 1 };

    let middle_x = width / 2;
    let middle_y = height / 2;

    for (index, label) in labels.iter().enumerate() {
        let i: u16 = index.try_into().unwrap();
        queue!(
            stdout(),
            MoveTo(
                middle_x - columns / 2 * (column_size + 1) + i % columns * (column_size + 1)
                    - if columns % 2 >= 1 {
                        (column_size + 1) / 2
                    } else {
                        0
                    },
                middle_y + i / columns - rows / 2
            ),
            Print(label)
        )?;
    }

    Ok(())
}

fn get_local_size(keybindings: &[Keybinding], style: &Styles) -> (u16, u16) {
    let column_size = get_local_column_size(keybindings, style);

    let label_amount: u16 = keybindings.len() as u16;

    let columns: u16 = (label_amount as f64).sqrt().ceil() as u16;
    let rows = label_amount / columns
        + if columns + label_amount / columns < label_amount {
            1
        } else {
            0
        }
        - if label_amount % columns > 0 { 0 } else { 1 };

    ((column_size + 1) * columns + 4, rows + 2)
}

fn get_size(keybindings: &[Keybinding], style: &Styles) -> (u16, u16) {
    let mut current = get_local_size(keybindings, style);

    for keybinding in keybindings {
        if let Commands::Menu(bindings) = &keybinding.commands {
            let (new_x, new_y) = get_size(bindings, style);
            current.0 = max(current.0, new_x);
            current.1 = max(current.1, new_y);
        }
    }

    current
}
