use std::error::Error;

mod app;
mod config;
mod lib;

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

    if lib::config_file_exists() {
        match lib::load_config() {
            Ok((e, p, t)) => {
                let user = config::User {
                    email: e,
                    password: p,
                    token: t,
                };
            }
            Err(_) => println!("Failed to load config."),
        }
    } else {
        match lib::config_file_create("<EMAIL>", "<PASSWORD>") {
            Ok(()) => (),
            Err(_) => eprintln!("Failed to create config file."),
        }
    }

    // What do for the different commands
    if let Some(ref matches) = matches.subcommand_matches("login") {
        if matches.is_present("email") && matches.is_present("password") {
            lib::config_file_create(
                matches.value_of("email").to_owned().unwrap(),
                matches.value_of("password").to_owned().unwrap(),
            )
            .unwrap();
        } else {
            println!("Failed to update config file. Make sure you provided username and password");
        }
    }

    if let Some(_) = matches.subcommand_matches("list") {
        println!("Printing list!!");
    }

    if let Some(_) = matches.subcommand_matches("reserve") {
        println!("Reserving machine");
    }

    if let Some(_) = matches.subcommand_matches("whoami") {
        match lib::get_email() {
            Ok(s) => println!("You are logged in as: {}", s),
            Err(_) => println!("An error has occured"),
        }
    }

    Ok(())
}
