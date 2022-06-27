#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    crossterm::terminal::enable_raw_mode()?;

    let task = pseudo_terminal_task::PseudoTerminalTask::new();
    drop(task);

    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
