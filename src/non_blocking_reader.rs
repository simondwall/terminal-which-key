use std::io::{Read, stdin};

use futures::executor::block_on;
use tokio::{
    spawn,
    sync::mpsc::{self, error::TryRecvError},
    task::JoinHandle,
};

use termion::{event::Event, input::TermReadEventsAndRaw};

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

pub struct NonBlockingEventAndRawReader {
    reader_handle: JoinHandle<()>,
    rx: mpsc::Receiver<(Event, Vec<u8>)>,
    disconnected: bool,
}

impl NonBlockingEventAndRawReader {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<(Event, Vec<u8>)>(100);
        let reader_handle = spawn(Self::read(tx));

        Self {
            reader_handle,
            rx,
            disconnected: false,
        }
    }

    pub fn is_eof(&self) -> bool {
        self.disconnected
    }

    pub fn read_available(&mut self) -> Vec<(Event, Vec<u8>)> {
        let mut buffer = Vec::new();
        while !self.disconnected {
            match self.rx.try_recv() {
                Ok(t) => buffer.push(t),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    self.disconnected = true;
                }
            }
        }
        buffer
    }

    async fn read(tx: mpsc::Sender<(Event, Vec<u8>)>) {
        for byte in stdin().events_and_raw() {
            let unwrapped_byte = byte.unwrap();
            match tx.try_send(unwrapped_byte.clone()) {
                Ok(_) => (),
                Err(mpsc::error::TrySendError::Closed(_)) => break,
                Err(mpsc::error::TrySendError::Full(_)) => {
                    // Retry send after 10 millis
                    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                    tx.try_send(unwrapped_byte).unwrap();
                }
            }
        }
    }
}

impl Drop for NonBlockingEventAndRawReader {
    fn drop(&mut self) {
        self.rx.close();
        let handle = std::mem::replace(&mut self.reader_handle, spawn(async {}));
        block_on(handle).unwrap();
    }
}
