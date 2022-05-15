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
    io::{stdout, Write},
    process::Command,
};

fn main() -> Result<()> {
    // Initialize
    enable_raw_mode()?;
    let (style, keybindings) = load_config();
    queue!(stdout(), EnterAlternateScreen, Hide)?;

    // Run
    let mut current_keybindings = &keybindings[..];
    loop {
        queue!(stdout(), Clear(ClearType::All))?;
        draw(current_keybindings, &style)?;
        stdout().flush()?;
        match read()? {
            Key(KeyEvent {
                code: KeyCode::Char(key),
                modifiers: KeyModifiers::NONE,
            }) => match run_command(current_keybindings, &key) {
                Some(new_keybindings) => current_keybindings = new_keybindings,
                None => break,
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
    execute!(stdout(), Show, LeaveAlternateScreen)?;
    Ok(())
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

    let middle_x = width / 2;
    let middle_y = height / 2;
    let columns: u16 = (label_amount as f64).sqrt().ceil() as u16;
    let rows = label_amount / columns
        + if columns + label_amount / columns < label_amount {
            1
        } else {
            0
        };

    for (index, label) in labels.iter().enumerate() {
        let i: u16 = index.try_into().unwrap();
        queue!(
            stdout(),
            MoveTo(
                middle_x - columns / 2 * (column_size + 1) + i % columns * (column_size + 1)
                    - if columns % 2 >= 1 { (column_size + 1) / 2 } else { 0 },
                middle_y + i / columns - rows / 2
            ),
            Print(label)
        )?;
    }

    Ok(())
}
