use anyhow::Result;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size};
use portable_pty::{native_pty_system, CommandBuilder, MasterPty, PtySize};
use tokio::sync::oneshot;
use std::io::{stdin, stdout, Read, Write};

#[tokio::main]
async fn main() -> Result<()> {
    enable_raw_mode()?;

    pty_forwarding_task().await?;

    disable_raw_mode()?;

    Ok(())
}

async fn pty_forwarding_task() -> Result<()> {
    let pty_system = native_pty_system();
    let (cols, rows) = size()?;

    // Create a new pty
    let pair = pty_system.openpty(PtySize {
        rows,
        cols,
        // Best practice to set these as well:
        pixel_width: 0,
        pixel_height: 0,
    })?;

    let slave = pair.slave;
    let master = pair.master;

    let cmd = CommandBuilder::new("fish");
    slave.spawn_command(cmd)?;
    
    let (tx, rx) = oneshot::channel();

    let print_handle = tokio::spawn(print_task(master.try_clone_reader()?));
    let input_handle = tokio::spawn(input_task(master, rx));

    print_handle.await??;
    tx.send(()).unwrap();
    input_handle.await??;

    Ok(())
}

async fn input_task(mut master: Box<dyn MasterPty + Send>, mut rx: oneshot::Receiver<()>) -> Result<()> {
    let mut input = stdin().bytes();
    loop {
        if rx.try_recv().is_ok() {
            break;
        }
        match input.next() {
            Some(Ok(3)) => break,
            Some(Ok(0)) => master.write(b"Hello, World!"),
            Some(Ok(byte)) => master.write(&[byte]),
            _ => Ok(0)
        }?;
    }

    Ok(())
}

async fn print_task(reader: Box<dyn Read + Send>) -> Result<()> {
    let mut out = stdout();
    for character in reader.bytes() {
        out.write_all(&[character.unwrap()])?;
        out.flush()?;
    }

    Ok(())
}
