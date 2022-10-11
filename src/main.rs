mod config;
mod non_blocking_reader;
mod pty;
mod window;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut command = portable_pty::CommandBuilder::new("fish");
    command.cwd(std::env::current_dir().unwrap());

    pty::PseudoTerminalBuilder::new(command)
        .with_callback("root", vec![0], |which_key| {
            let win = window::Window::new();
            win.draw_border(10, 50);
            which_key.set_mode("menu");
        })
        .with_callback("menu", vec![27], |which_key| {
            which_key.redraw_screen();
            which_key.set_mode("root");
        })
        .build()
        .run();

    Ok(())
}
