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

    let mut user = config::User::new();

    if lib::config_file_exists() {
        match lib::load_config() {
            Ok((e, p, t)) => {
                user = config::User {
                    email: e,
                    password: p,
                    token: t,
                };
            }
            Err(_) => println!("Failed to load config."),
        }
    } else {
        match lib::config_file_create("<EMAIL>", "<PASSWORD>") {
            Ok(()) => println!("Log in was a success."),
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
        match lib::get_machines(&user.token) {
            Ok(m) => println!("{:?}", m),
            Err(_) => println!("An error occured while trying to print the machines."),
        }
    }

    if let Some(_) = matches.subcommand_matches("reserve") {
        println!("Reserving machine");
    }

    if let Some(ref matches) = matches.subcommand_matches("whoami") {
        println!("You are logged in as: {}", user.email);
        if matches.is_present("secrets") {
            println!("Password: {}", user.password);
            println!("Token: {}", user.token);
        }
    }

    Ok(())
}
