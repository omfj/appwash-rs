mod app;
mod config;

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

#[warn(unused_variables)]
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let matches = app::create_app().get_matches();

    Ok(())
}
