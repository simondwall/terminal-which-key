use std::io::Write;

use portable_pty::CommandBuilder;

use crate::{
    config::{load_config_from_file, Child, Config, MenuConfig},
    non_blocking_reader::{NonBlockingEventAndRawReader, NonBlockingReader},
    window::Window,
};

use termion::event::{Event::Key as EventKey, Key};

pub struct PseudoTerminal {
    master: Box<dyn portable_pty::MasterPty + Send>,
    parser: vt100::Parser,
    config: Config<'static>,
}

impl PseudoTerminal {
    pub fn new(config_path: &str) -> Self {
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

        let config =
            load_config_from_file(config_path, pair.master.try_clone_writer().unwrap()).unwrap();

        let mut command = CommandBuilder::new(&config.shell_path);
        command.cwd(std::env::current_dir().unwrap());
        pair.slave.spawn_command(command).unwrap();
        let parser = vt100::Parser::new(rows, cols, 0);

        Self {
            master: pair.master,
            parser,
            config,
        }
    }

    pub fn run(self) {
        crossterm::terminal::enable_raw_mode().unwrap();
        std::io::stdout()
            .write_all(ansi_escapes::ClearScreen.to_string().as_bytes())
            .unwrap();

        let mut master_reader = NonBlockingReader::new(self.master.try_clone_reader().unwrap());
        let mut stdin_reader = NonBlockingEventAndRawReader::new();

        let root_menu: MenuConfig = MenuConfig {
            key: Key::Esc,
            name: "__root".to_string(),
            children: vec![Child::Menu(self.config.menu_config)],
            condition: None,
            description: None,
        };
        let mut current_menu = &root_menu;
        let mut screen = self.parser.screen().clone().contents_formatted();

        let mut master_writer = self.master.try_clone_writer().unwrap();
        let parser = std::sync::Arc::new(std::sync::Mutex::new(self.parser));

        let winch_parser = parser.clone();
        let mut winch_stream =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::window_change()).unwrap();
        tokio::spawn(async move {
            loop {
                winch_stream.recv().await;
                let (cols, rows) = crossterm::terminal::size().unwrap();
                self.master
                    .resize(portable_pty::PtySize {
                        rows,
                        cols,
                        pixel_width: 0,
                        pixel_height: 0,
                    })
                    .unwrap();
                winch_parser.lock().unwrap().set_size(rows, cols);
            }
        });

        while !stdin_reader.is_eof() && !master_reader.is_eof() {
            let data = stdin_reader.read_available();

            for (event, bytes) in data {
                if let EventKey(key) = event {
                    if &current_menu.name != "__root" && key == Key::Esc {
                        current_menu = &root_menu;
                        Self::redraw_screen(&screen);
                    } else if let Some(child) = current_menu
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
                                screen =
                                    parser.lock().unwrap().screen().clone().contents_formatted();
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
                        master_writer.write_all(&bytes).unwrap();
                        master_writer.flush().unwrap();
                    }
                } else if current_menu.name == "__root" {
                    master_writer.write_all(&bytes).unwrap();
                    master_writer.flush().unwrap();
                }
            }

            let master_data = &master_reader.read_available()[..];
            std::io::stdout().write_all(master_data).unwrap();
            std::io::stdout().flush().unwrap();
            parser.lock().unwrap().process(master_data);
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
}
