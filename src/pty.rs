use std::io::Write;

use crate::non_blocking_reader::NonBlockingReader;

type CallbackFunction = fn(&mut WhichKeyFunctions);

#[derive(Hash, PartialEq)]
struct ModeAndKey {
    key: Vec<u8>,
    mode: String,
}
impl Eq for ModeAndKey {}

pub struct PseudoTerminalBuilder {
    command: portable_pty::CommandBuilder,
    callbacks: std::collections::HashMap<ModeAndKey, CallbackFunction>,
}

impl PseudoTerminalBuilder {
    pub fn new(command: portable_pty::CommandBuilder) -> Self {
        Self {
            command,
            callbacks: std::collections::HashMap::new(),
        }
    }

    pub fn with_callback(mut self, mode: &str, key: Vec<u8>, callback: CallbackFunction) -> Self {
        self.callbacks.insert(
            ModeAndKey {
                key,
                mode: mode.to_string(),
            },
            callback,
        );
        self
    }

    pub fn build(self) -> PseudoTerminal {
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
        pair.slave.spawn_command(self.command).unwrap();
        let parser = vt100::Parser::new(rows, cols, 0);

        PseudoTerminal {
            master: pair.master,
            callbacks: self.callbacks,
            parser,
        }
    }
}

pub struct PseudoTerminal {
    master: Box<dyn portable_pty::MasterPty>,
    callbacks: std::collections::HashMap<ModeAndKey, CallbackFunction>,
    parser: vt100::Parser,
}

impl PseudoTerminal {
    pub fn run(mut self) {
        crossterm::terminal::enable_raw_mode().unwrap();
        std::io::stdout()
            .write_all(ansi_escapes::ClearScreen.to_string().as_bytes())
            .unwrap();

        let mut master_reader = NonBlockingReader::new(self.master.try_clone_reader().unwrap());
        let mut stdin_reader = NonBlockingReader::new(Box::new(std::io::stdin()));

        let mut which_key_functions = WhichKeyFunctions::new(Vec::new(), "root".to_string());

        while !stdin_reader.is_eof() && !master_reader.is_eof() {
            let data = stdin_reader.read_available();
            if let Some(function) = self.callbacks.get(&ModeAndKey {
                key: data.clone(),
                mode: which_key_functions.mode.clone(),
            }) {
                which_key_functions.screen_content =
                    self.parser.screen().clone().contents_formatted();
                function(&mut which_key_functions);
            } else if which_key_functions.mode == "root" {
                self.master.write_all(&data).unwrap();
                self.master.flush().unwrap();
            }

            let master_data = &master_reader.read_available()[..];
            std::io::stdout().write_all(master_data).unwrap();
            std::io::stdout().flush().unwrap();
            self.parser.process(master_data);
        }

        WhichKeyFunctions::new(Vec::new(), String::new()).end();
    }
}

pub struct WhichKeyFunctions {
    screen_content: Vec<u8>,
    mode: String,
}
impl WhichKeyFunctions {
    fn new(screen_content: Vec<u8>, mode: String) -> Self {
        WhichKeyFunctions {
            screen_content,
            mode,
        }
    }

    pub fn end(&self) {
        crossterm::terminal::disable_raw_mode().unwrap();
        std::process::exit(0);
    }

    pub fn redraw_screen(&self) {
        std::io::stdout()
            .write_all(ansi_escapes::ClearScreen.to_string().as_bytes())
            .unwrap();
        std::io::stdout().write_all(&self.screen_content).unwrap();
        std::io::stdout().flush().unwrap();
    }

    pub fn set_mode(&mut self, mode: &str) {
        self.mode = mode.to_owned();
    }
}
