use crate::lib::pretty;
use colored::Colorize;
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

    // Load config or create a new one
    if lib::config_file_exists() {
        match lib::load_config() {
            Ok((email, password, location, token)) => {
                user = config::User {
                    email,
                    password,
                    token,
                    location,
                };
            }
            Err(_) => {
                println!("Failed to load config.")
            }
        }
    } else {
        match lib::config_file_create("<EMAIL>", "<PASSWORD>") {
            Ok(()) => println!("Created config file in .config."),
            Err(_) => eprintln!("Failed to create config file."),
        }
    }

    // What do for the different commands

    // Command: login
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

    // Command: list
    if let Some(_) = matches.subcommand_matches("list") {
        match lib::get_machines(&user.token, &user.location) {
            Ok(machines) => match pretty::machines(machines, &user.location, &user.token) {
                Ok(()) => (),
                Err(_) => println!("Failed to list machines."),
            },
            Err(e) => println!("An error occured trying to get the machines. Error: {}", e),
        }
    }

    // Command: balance
    if let Some(_) = matches.subcommand_matches("balance") {
        match lib::get_balance(&user.token) {
            Ok((b, c)) => println!("You balance is: {} {}", format!("{}", b).green(), c),
            Err(_) => println!("An error occured trying to get your balance."),
        }
    }

    // Command: stop
    if let Some(ref matches) = matches.subcommand_matches("stop") {
        let machine_id = matches.value_of("id").unwrap().parse().unwrap();

        match lib::stop_machine(&user.token, &machine_id) {
            Ok(resp) => {
                let status_code = resp.errorCode;
                if status_code == 0 {
                    println!("{}", format!("Machine {} stopped.", machine_id).green());
                } else {
                    println!(
                        "Failed to stop machine {}. Returned code: {}",
                        format!("{}", machine_id).yellow(),
                        status_code,
                    );
                    println!("Description: {}", resp.errorDescription);
                    println!(
                        "{}",
                        format!("Are you sure that machine is stoppable?").red()
                    );
                }
            }
            Err(_) => println!(
                "Failed to reserve machine {}.",
                format!("{}", machine_id).yellow()
            ),
        }
    }

    // Command: reserve
    if let Some(ref matches) = matches.subcommand_matches("reserve") {
        let machine_id = matches.value_of("id").unwrap().parse().unwrap();

        match lib::reserve_machine(&user.token, &machine_id) {
            Ok(resp) => {
                let status_code = resp.errorCode;
                if status_code == 0 {
                    println!("{}", format!("Machine {} reserved.", machine_id).green());
                } else {
                    println!(
                        "Failed to reserve machine {}. Returned code: {}",
                        format!("{}", machine_id).yellow(),
                        status_code,
                    );
                    println!("Description: {}", resp.errorDescription);
                    println!(
                        "{}",
                        format!("Are you sure that machine is reservable?").red()
                    );
                }
            }
            Err(_) => println!(
                "Failed to reserve machine {}.",
                format!("{}", machine_id).yellow()
            ),
        }
    }

    // Command: history
    if let Some(_) = matches.subcommand_matches("history") {
        match lib::get_history(&user.token) {
            Ok(m) => match pretty::history(m) {
                Ok(()) => (),
                Err(_) => println!("{}", format!("Failed to list history.").red()),
            },
            Err(_) => println!("{}", format!("Failed to get history.").red()),
        }
    }

    // Command: whoami
    if let Some(ref matches) = matches.subcommand_matches("whoami") {
        println!(
            "You are logged in as: {}",
            format!("{}", &user.email).green()
        );
        println!(
            "Your location is: {}",
            format!("{}", &user.location).green()
        );
        if !matches.is_present("secrets") {
            println!(
                "Run with {} to see your password and token.",
                format!("{}", "--secrets").yellow()
            );
        }
        if matches.is_present("secrets") {
            println!("Password: {}", format!("{}", &user.password).green());
            println!("Token: {}", format!("{}", &user.token).green());
        }
    }

    // Command: place
    if let Some(ref matches) = matches.subcommand_matches("location") {
        if let Some(ref matches) = matches.subcommand_matches("change") {
            let new_location = matches.value_of("location").unwrap().parse().unwrap();
            match lib::change_location(new_location) {
                Ok(()) => println!("{}", format!("Location changed.").green()),
                Err(_) => println!("{}", format!("Failed to change location.").red()),
            }
        } else {
            match lib::get_location_info(&user.token, &user.location) {
                Ok(info) => match pretty::location_info(info.data) {
                    Ok(()) => (),
                    Err(_) => println!("{}", format!("Failed to list location info.").red()),
                },
                Err(_) => println!("{}", format!("Failed to get location info.").red()),
            }
        }
    }

    Ok(())
}
