mod config;
mod keys;
mod non_blocking_reader;
mod pty;
mod window;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut command = portable_pty::CommandBuilder::new("fish");
    command.cwd(std::env::current_dir().unwrap());

    let config = config::load_config_from_file(&get_config_path().expect("There is no configuration file either under $THERMINAL_WHICH_KEY_CONFIG, ~/.config/terminal-which-key/init.lua or ~/.terminal-which-key/init.lua")).unwrap();

    let pseudo_terminal = pty::PseudoTerminal::new(config);
    pseudo_terminal.run();

    Ok(())
}

fn get_config_path() -> Option<String> {
    let prio1 = std::env::var("TERMINAL_WHICH_KEY_CONFIG").unwrap_or("".to_string());
    let prio2 = format!(
        "{}/.config/terminal-which-key/init.lua",
        std::env::var("HOME").unwrap()
    );
    let prio3 = format!(
        "{}/terminal-which-key/init.lua",
        std::env::var("HOME").unwrap()
    );

    if std::path::Path::new(&prio1).exists() {
        Some(prio1)
    } else if std::path::Path::new(&prio2).exists() {
        Some(prio2)
    } else if std::path::Path::new(&prio3).exists() {
        Some(prio3)
    } else {
        None
    }
}
