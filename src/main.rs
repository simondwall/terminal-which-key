mod config;
mod keys;
mod non_blocking_reader;
mod pty;
mod window;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut command = portable_pty::CommandBuilder::new("fish");
    command.cwd(std::env::current_dir().unwrap());

    let _config = config::load_config_from_file(get_config_path()).unwrap();

    Ok(())
}

fn get_config_path() -> &'static str {
    // TODO lookup with env variable > ~/.config directory > ~ directory > default config

    "../test_files/complex_config.lua"
}
