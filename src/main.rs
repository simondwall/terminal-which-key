use anyhow::Result;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use portable_pty::{native_pty_system, CommandBuilder, PtySize /*PtySystem*/};
use std::io::{stdin, BufRead, BufReader, Read};

#[tokio::main]
async fn main() -> Result<()> {
    enable_raw_mode()?;

    // Use the native pty implementation for the system
    let pty_system = native_pty_system();

    // Create a new pty
    let mut pair = pty_system
        .openpty(PtySize {
            rows: 24,
            cols: 80,
            // Not all systems support pixel_width, pixel_height,
            // but it is good practice to set it to something
            // that matches the size of the selected font.  That
            // is more complex than can be shown here in this
            // brief example though!
            pixel_width: 0,
            pixel_height: 0,
        })
        .unwrap();

    // Spawn a shell into the pty
    let cmd = CommandBuilder::new("fish");
    /* let child = */
    pair.slave.spawn_command(cmd).unwrap();

    // Send data to the pty by writing to the master
    let input_task = tokio::spawn(async move {
        for character in stdin().bytes() {
            match character.unwrap() {
                3 => break,
                c => pair.master.write(&[c]), //writeln!(pair.master, "{}", c),
            }.unwrap();
        }
    });

    // Read and parse output from the pty with reader
    let reader = pair.master.try_clone_reader().unwrap();
    let buffered_reader = BufReader::new(reader);

    let print_task = tokio::spawn(async move {
        for line in buffered_reader.lines() {
            println!("{}", line.unwrap());
        }
    });

    input_task.await?;
    print_task.await?;

    disable_raw_mode()?;

    Ok(())
}
