use chrono::{prelude::*, Duration};
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
        match lib::get_machines(&user.token) {
            Ok(m) => match pretty_machines(m) {
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
        if matches.is_present("machine") {
            let machine_id = matches.value_of("machine").unwrap().parse().unwrap();

            match lib::stop_machine(&user.token, machine_id) {
                Ok(c) => {
                    if c == 0 {
                        println!("Machine {} stopped.", machine_id);
                    } else {
                        println!("Failed to stop machine {}.", machine_id);
                    }
                }
                Err(_) => println!("Failed to stop machine {}.", machine_id),
            }
        }
    }

    // Command: reserve
    if let Some(_) = matches.subcommand_matches("reserve") {
        if matches.is_present("machine") {
            let machine_id = matches.value_of("machine").unwrap().parse().unwrap();

            match lib::reserve_machine(&user.token, machine_id) {
                Ok(c) => {
                    if c == 0 {
                        println!("Machine {} reserved.", machine_id);
                    } else {
                        println!("Failed to reserve machine {}.", machine_id);
                    }
                }
                Err(_) => println!("Failed to reserve machine {}.", machine_id),
            }
        }
    }

    // Command: whoami
    if let Some(ref matches) = matches.subcommand_matches("whoami") {
        println!(
            "You are logged in as: {}",
            format!("{}", &user.email).green()
        );
        if matches.is_present("secrets") {
            println!("Password: {}", format!("{}", &user.password).green());
            println!("Token: {}", format!("{}", &user.token).green());
        }
    }

    Ok(())
}

fn pretty_machines(machines: lib::Machines) -> Result<(), Box<dyn Error>> {
    for machine in machines.data {
        let id = machine.externalId;
        let state = machine.state;

        match state.as_str() {
            "AVAILABLE" => println!("{}", format!("{} - {}", id, state).green()),
            "STOPPABLE" | "OCCUPIED" => {
                let s = machine.lastSessionStart.unwrap().into();
                let naive = NaiveDateTime::from_timestamp(s, 0);
                // from naive to local
                let local =
                    Local::now()
                        .date()
                        .and_hms(naive.hour(), naive.minute(), naive.second())
                        + Duration::hours(2); // FROM UTC TO UTC+2 in a bad way must fix
                let start_time = local.format("%H:%M");
                println!(
                    "{}",
                    format!("{id} - {state} | Started: {start_time}").yellow()
                );
            }
            "FAULTED" => println!("{}", format!("{} - {}", id, state).red().blink()),
            _ => println!("{}", format!("{} - {}", id, state).yellow()),
        }
    }

    Ok(())
}
