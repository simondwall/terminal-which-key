#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

use std::{
    io::{stdin, stdout, Read, Write},
    mem,
};

use crossterm::terminal::size;
use nonblock::NonBlockingReader;
use portable_pty::{native_pty_system, CommandBuilder, MasterPty, PtySize};
use tokio::{sync::oneshot, task::JoinHandle};

pub struct PseudoTerminalTask {
    print_handle: JoinHandle<()>,
    input_handle: JoinHandle<()>,
}

impl PseudoTerminalTask {
    pub fn new() -> Self {
        let pty_system = native_pty_system();
        let (cols, rows) = size().unwrap();

        // Create a new pty
        let pair = pty_system
            .openpty(PtySize {
                rows,
                cols,
                // Best practice to set these as well:
                pixel_width: 0,
                pixel_height: 0,
            })
            .unwrap();

        let slave = pair.slave;
        let master = pair.master;

        let cmd = CommandBuilder::new("fish");
        slave.spawn_command(cmd).unwrap();

        let (tx, rx) = oneshot::channel();

        let print_handle = tokio::spawn(PseudoTerminalTask::print_task(
            master.try_clone_reader().unwrap(),
            tx,
        ));
        let input_handle = tokio::spawn(PseudoTerminalTask::input_task(master, rx));

        Self {
            print_handle,
            input_handle,
        }
    }

    pub fn register_callback() {
        todo!()
    }

    async fn print_task(reader: Box<dyn Read + Send>, tx: oneshot::Sender<u8>) {
        let mut out = stdout();
        for character in reader.bytes() {
            out.write_all(&[character.unwrap()]).unwrap();
            out.flush().unwrap();
        }

        tx.send(0).unwrap();
    }

    async fn input_task(mut master: Box<dyn MasterPty + Send>, mut rx: oneshot::Receiver<u8>) {
        let mut nonblocking_reader = NonBlockingReader::from_fd(stdin()).unwrap();

        while !nonblocking_reader.is_eof() {
            let mut data = vec![];
            nonblocking_reader.read_available(&mut data).unwrap();

            for byte in data {
                match byte {
                    1 => master.write(b"Hello, World!"),
                    byte => master.write(&[byte]),
                }
                .unwrap();
            }

            if rx.try_recv().is_ok() {
                break;
            }
        }
    }
}

impl Drop for PseudoTerminalTask {
    fn drop(&mut self) {
        let print_handle = mem::replace(&mut self.print_handle, tokio::spawn(async {}));
        futures::executor::block_on(print_handle).unwrap();
        let input_handle = mem::replace(&mut self.input_handle, tokio::spawn(async {}));
        futures::executor::block_on(input_handle).unwrap();
    }
}
