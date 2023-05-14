use http::AppwashClient;
use user::Token;
use views::Views;

use crate::{models::MachineState, user::UserConfig};

mod app;
mod http;
mod models;
mod user;
mod views;

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

fn run() -> Result<(), String> {
    let app = app::get_app();
    let matches = app.get_matches();
    let user = UserConfig::load().expect("Failed to load user config");
    let mut client = AppwashClient::new(user).expect("Failed to create the appwash client");
    let resp = client.login().expect("Failed to log in...");

    client.user.token = Token::new(resp.login.token, resp.token_expire_ts);

    if let Some(m) = matches.subcommand_matches("reserve") {
        let id = m.get_one::<usize>("id").expect("Could not get machine id");

        return Err("Not implemented yet")?;
    }

    if let Some(m) = matches.subcommand_matches("stop") {
        let id = m.get_one::<usize>("id").unwrap();

        return Err("Not implemented yet")?;
    }

    if let Some(_) = matches.subcommand_matches("balance") {
        let balance = client.get_balance().expect("Failed to get balance");

        println!("Balance: {}", balance);

        return Ok(());
    }

    if let Some(m) = matches.subcommand_matches("me") {
        let with_secrets = m.get_flag("secrets");

        Views::render_profile(client.user, with_secrets);

        return Ok(());
    }

    if let Some(m) = matches.subcommand_matches("list") {
        let available = m.get_flag("available");
        let occupied = m.get_flag("occupied");
        let stoppable = m.get_flag("stoppable");

        let machines = client.get_machines().expect("Failed to get machines");

        let machines = machines
            .data
            .into_iter()
            .filter(|machine| {
                if available {
                    machine.state == MachineState::Available
                } else if occupied {
                    machine.state == MachineState::Occupied
                } else if stoppable {
                    machine.state == MachineState::Stoppable
                } else {
                    true
                }
            })
            .collect::<Vec<_>>();

        Views::render_machines(machines);

        return Ok(());
    }

    if let Some(_) = matches.subcommand_matches("location") {
        let location = client.get_location_info().expect("Failed to get location");

        println!("Location: {:#?}", location);

        return Ok(());
    }

    if let Some(_) = matches.subcommand_matches("history") {
        let history = client.get_history().expect("Failed to get history");

        println!("History: {:#?}", history);

        return Ok(());
    }

    Ok(())
}
