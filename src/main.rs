mod keybinding;
mod pty;
mod window;
mod nbr;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    pty::PseudoTerminalBuilder::new(portable_pty::CommandBuilder::new("fish"))
        .with_callback(0, |_| {
            let win = window::Window::new(Vec::new());
            win.draw_border(0, 0);
        })
        .with_callback(1, |terminal| {
            terminal.redraw_screen();
        })
        .build()
        .run();

    Ok(())
}
