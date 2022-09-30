use std::error::Error;

mod api;
mod app;

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
    let matches = app::create_app().get_matches();

    Ok(())
}
