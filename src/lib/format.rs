use std::error::Error;

use chrono::{Duration, NaiveDateTime};
use colored::Colorize;
use tabled::{Style, Table, Tabled};

use crate::lib::api;

use super::models::{History, LocationInfo, MachineData, Response};

#[derive(Tabled)]
struct Purchases {
    description: String,
    date: String,
    amount: String,
    balance: String,
}

pub fn machines(
    machines: Response<Vec<MachineData>>,
    location: &u32,
    token: &String,
) -> Result<(), Box<dyn Error>> {
    let location_info = api::get_location_info(token, location)?;
    println!(
        "{}",
        format!("{}\n", location_info.data.name).green().underline()
    );

    println!("{:7} {}", "ID".green().bold(), "Status".green().bold());
    for machine in machines.data {
        let id = machine.externalId;
        let state = machine.state;

        match state.as_str() {
            "AVAILABLE" => println!(
                "{:5} - {}",
                format!("{id}").bold(),
                format!("{state}").green()
            ),
            "STOPPABLE" | "OCCUPIED" => {
                let start_time = {
                    let date = NaiveDateTime::from_timestamp(machine.lastSessionStart.unwrap(), 0)
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
            "FAULTED" => println!("{}", format!("{:5} - {}", id, state).red().blink()),
            _ => println!(
                "{} - {}",
                format!("{id}").bold(),
                format!("{state}").yellow()
            ),
        }
    }

    Ok(())
}

pub fn history(machines: Response<Vec<History>>) -> Result<(), Box<dyn Error>> {
    let mut purchases: Vec<Purchases> = Vec::new();

    for purchase in machines.data {
        let _type = purchase.serviceType;
        let description = purchase.mutationDescription;
        let currency = purchase.currency;
        let amount = purchase.mutationCents / 100;
        let balance = purchase.balanceCentsAfter / 100;
        let time = {
            let date = NaiveDateTime::from_timestamp(purchase.mutationTimestamp as i64, 0)
                + Duration::hours(2);
            let formatted_date = date.format("%d %b %Y %H:%M");
            formatted_date.to_string()
        };

        purchases.push(Purchases {
            description,
            date: time,
            amount: format!("{} {}", amount, currency),
            balance: format!("{} {}", balance, currency),
        });
    }

    let table = Table::new(purchases).with(Style::modern()).to_string();

    println!("{}", table);

    Ok(())
}

pub fn location_info(info: LocationInfo) -> Result<(), Box<dyn Error>> {
    let name = info.name;
    let id = info.externalId;
    let _type = info.locationTypeObject.name;

    println!("Name: {}", format!("{name}").green());
    println!("ID: {}", format!("{id}").green());
    println!("Type: {}", format!("{_type}").green());

    Ok(())
}
