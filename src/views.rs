use chrono::{Duration, NaiveDateTime};
use colored::Colorize;

use crate::{
    models::{MachineData, MachineState},
    user::UserConfig,
};

pub struct Views;

impl Views {
    pub fn render_machines(machines: Vec<MachineData>) {
        for machine in machines {
            let id = machine.external_id;
            let state = machine.state;

            match state {
                MachineState::Available => println!(
                    "{:5} - {}",
                    format!("{id}").bold(),
                    format!("{state}").green()
                ),
                MachineState::Stoppable | MachineState::Occupied => {
                    let start_time = {
                        let date = NaiveDateTime::from_timestamp_opt(
                            machine.last_session_start.unwrap(),
                            0,
                        )
                        .unwrap()
                            + Duration::hours(2);
                        let formatted_date = date.format("%H:%M");
                        formatted_date.to_string()
                    };
                    println!(
                        "{:5} - {}",
                        format!("{id}").bold(),
                        format!("{state} | Started: {start_time}").yellow()
                    );
                }
                MachineState::Faulted => {
                    println!("{}", format!("{:5} - {}", id, state).red().blink())
                }
                _ => println!(
                    "{} - {}",
                    format!("{id}").bold(),
                    format!("{state}").yellow()
                ),
            }
        }
    }

    pub fn render_profile(user: UserConfig, with_secrets: bool) {
        println!("Email: {}", user.account.email);
        println!("Location: {}", user.account.location);

        if with_secrets {
            println!("Password: {}", user.account.password);
            println!("Token secret: {}", user.token.secret);
            println!("Token expires: {}", user.token.expires);
        }
    }
}
