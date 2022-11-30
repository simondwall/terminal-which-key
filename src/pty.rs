use std::io::Write;

use portable_pty::CommandBuilder;

use crate::{
    config::{Child, Config, MenuConfig},
    keys::{Key, Symbol},
    non_blocking_reader::NonBlockingReader, window::Window,
};

pub struct PseudoTerminal {
    master: Box<dyn portable_pty::MasterPty + Send>,
    parser: vt100::Parser,
    config: Config<'static>,
}

impl PseudoTerminal {
    pub fn new(config: Config<'static>) -> Self {
        let (cols, rows) = crossterm::terminal::size().unwrap_or((80, 24));
        let pair = portable_pty::native_pty_system()
            .openpty(portable_pty::PtySize {
                rows,
                cols,
                // TODO set those as well
                pixel_width: 0,
                pixel_height: 0,
            })
            .unwrap();
        pair.slave
            .spawn_command(CommandBuilder::new(&config.shell_path))
            .unwrap();
        let parser = vt100::Parser::new(rows, cols, 0);

        Self {
            master: pair.master,
            parser,
            config,
        }
    }

    pub fn run(mut self) {
        crossterm::terminal::enable_raw_mode().unwrap();
        std::io::stdout()
            .write_all(ansi_escapes::ClearScreen.to_string().as_bytes())
            .unwrap();

        let mut master_reader = NonBlockingReader::new(self.master.try_clone_reader().unwrap());
        let mut stdin_reader = NonBlockingReader::new(Box::new(std::io::stdin()));

        let root_menu: MenuConfig = MenuConfig {
            key: Key {ctrl: true, opt: false, shift: false, symbol: Symbol::LeftBracket},
            name: "__root".to_string(),
            children: vec![Child::Menu(self.config.menu_config)],
            condition: None,
            description: None,
        };
        let mut current_menu = &root_menu;
        let mut screen = self.parser.screen().clone().contents_formatted();

        while !stdin_reader.is_eof() && !master_reader.is_eof() {
            let data = stdin_reader.read_available();

            if let Ok(key) = Key::from_slice(&data) {
                if &current_menu.name != "__root" && key == Key::from_str("<Esc>").unwrap() {
                    current_menu = &root_menu;
                    Self::redraw_screen(&screen);
                }
                else if let Some(child) = current_menu
                    .children
                    .iter()
                    .filter(|c| match c {
                        Child::Menu(m) => m.key == key,
                        Child::Action(a) => a.key == key,
                    })
                    .last()
                {
                    match child {
                        Child::Menu(menu) => {
                            screen = self.parser.screen().clone().contents_formatted();
                            Self::redraw_screen(&screen);
                            current_menu = menu;
                            Window::new(&current_menu).draw();
                        }
                        Child::Action(action) => {
                            action
                                .action
                                .call::<_, ()>(())
                                .expect("Lua function threw an error");
                            current_menu = &root_menu;
                            Self::redraw_screen(&screen);
                        }
                    }
                } else if current_menu.name == "__root" {
                    self.master.write_all(&data).unwrap();
                    self.master.flush().unwrap();
                }
            } else if current_menu.name == "__root" {
                self.master.write_all(&data).unwrap();
                self.master.flush().unwrap();
            }

            let master_data = &master_reader.read_available()[..];
            std::io::stdout().write_all(master_data).unwrap();
            std::io::stdout().flush().unwrap();
            self.parser.process(master_data);
        }

        crossterm::terminal::disable_raw_mode().unwrap();
        std::process::exit(0);
    }

    #[allow(dead_code)]
    fn redraw_screen(screen_content: &Vec<u8>) {
        std::io::stdout()
            .write_all(ansi_escapes::ClearScreen.to_string().as_bytes())
            .unwrap();
        std::io::stdout().write_all(&screen_content).unwrap();
        std::io::stdout().flush().unwrap();
    }

    #[allow(dead_code)]
    fn update_size(&self) {
        todo!();
    }
}
