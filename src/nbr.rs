use std::io::Read;

use futures::executor::block_on;
use tokio::{
    spawn,
    sync::mpsc::{self, error::TryRecvError},
    task::JoinHandle,
};

pub struct NonBlockingReader {
    reader_handle: JoinHandle<()>,
    rx: mpsc::Receiver<u8>,
    empty: bool,
}

impl NonBlockingReader {
    pub fn new(reader: Box<dyn Read + Send>) -> Self {
        let (tx, rx) = mpsc::channel::<u8>(100);
        let reader_handle = spawn(NonBlockingReader::read(reader, tx));

        NonBlockingReader {
            reader_handle,
            rx,
            empty: false,
        }
    }

    pub fn is_eof(&self) -> bool {
        self.empty
    }

    pub fn read_available(&mut self) -> Vec<u8> {
        let mut buffer = Vec::new();
        while !self.empty {
            match self.rx.try_recv() {
                Ok(byte) => buffer.push(byte),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    self.empty = true;
                }
            }
        }
        buffer
    }

    async fn read(reader: Box<dyn Read + Send>, tx: mpsc::Sender<u8>) {
        for byte in reader.bytes() {
            tx.send(byte.unwrap()).await.unwrap();
        }
    }
}

impl Drop for NonBlockingReader {
    fn drop(&mut self) {
        let handle = std::mem::replace(&mut self.reader_handle, spawn(async {}));
        block_on(handle).unwrap();
    }
}
