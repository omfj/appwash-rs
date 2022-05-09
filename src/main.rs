#![allow(unused_imports)]

use colored::Colorize;
use serde_json::Value;
use std::{collections::HashMap, error::Error};

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
            Err(_) => println!("An error occured trying to get the machines."),
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
            lib::stop_machine(machine_id).unwrap()
        }
    }

    // Command: reserve
    if let Some(_) = matches.subcommand_matches("reserve") {
        println!("Reserving machine");
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
        println!("{} - {}", id, state);
    }

    Ok(())
}
