use std::io::Write;

use crate::nbr::NonBlockingReader;

pub struct PseudoTerminalBuilder {
    command: portable_pty::CommandBuilder,
    callbacks: std::collections::HashMap<u8, fn(&mut PseudoTerminal)>,
}

impl PseudoTerminalBuilder {
    pub fn new(command: portable_pty::CommandBuilder) -> Self {
        Self {
            command,
            callbacks: std::collections::HashMap::new(),
        }
    }

    pub fn with_callback(mut self, key: u8, callback: fn(&mut PseudoTerminal)) -> Self {
        self.callbacks.insert(key, callback);
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
            stdout: std::io::stdout(),
            master: pair.master,
            callbacks: self.callbacks,
            parser,
        }
    }
}

pub struct PseudoTerminal {
    stdout: std::io::Stdout,
    master: Box<dyn portable_pty::MasterPty>,
    callbacks: std::collections::HashMap<u8, fn(&mut PseudoTerminal)>,
    parser: vt100::Parser,
}

impl PseudoTerminal {
    pub fn run(mut self) {
        self.start();

        let mut master_reader = NonBlockingReader::new(self.master.try_clone_reader().unwrap());
        let mut stdin_reader = NonBlockingReader::new(Box::new(std::io::stdin()));

        while !stdin_reader.is_eof() && !master_reader.is_eof() {
            for byte in stdin_reader.read_available() {
                if self.callbacks.contains_key(&byte) {
                    self.callbacks.get(&byte).unwrap()(&mut self);
                } else {
                    self.master.write_all(&[byte]).unwrap();
                    self.master.flush().unwrap();
                }
            }

            let master_data = &master_reader.read_available()[..];
            self.stdout.write_all(master_data).unwrap();
            self.stdout.flush().unwrap();
            self.parser.process(master_data);
        }

        self.end();
    }

    fn start(&mut self) {
        crossterm::terminal::enable_raw_mode().unwrap();
        crossterm::execute!(self.stdout, crossterm::terminal::EnterAlternateScreen).unwrap();
    }

    pub fn end(&mut self) {
        crossterm::terminal::disable_raw_mode().unwrap();
        crossterm::execute!(self.stdout, crossterm::terminal::LeaveAlternateScreen).unwrap();
        std::process::exit(0);
    }

    pub fn redraw_screen(&mut self) {
        self.stdout
            .write_all(ansi_escapes::ClearScreen.to_string().as_bytes())
            .unwrap();
        self.stdout
            .write_all(&self.parser.screen().contents_formatted())
            .unwrap();
        self.stdout.flush().unwrap();
    }
}
