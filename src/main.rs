use std::error::Error;

use crate::user::UserConfig;

mod api;
mod app;
mod user;

fn main() {
    let result = run();

    match result {
        Ok(()) => {
            std::process::exit(0);
        }
        Err(err) => {
            eprintln!("Exited with error: {:#}", err);
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    // Load or create config
    match UserConfig::load() {
        Ok(_) => (),
        Err(_) => UserConfig::create_default_config(),
    }

    // Create CLAP app
    let app = app::create_app();
    let matches = app.get_matches();

    Ok(())
}
