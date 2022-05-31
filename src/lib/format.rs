use std::error::Error;

use chrono::{Duration, NaiveDateTime};
use colored::Colorize;
use tabled::{
    object::{Columns, Segment},
    Alignment, Modify, Style, Table, Tabled,
};

use crate::lib::api;

use super::models::{History, LocationInfo, MachineData, Response};

#[derive(Tabled)]
struct HistoryTableRow {
    #[tabled(rename = "Type")]
    _type: String,
    #[tabled(rename = "📝Description")]
    description: String,
    #[tabled(rename = "💲Amount")]
    amount: String,
    #[tabled(rename = "📅Date")]
    date: String,
}

pub fn machines(
    machines: Response<Vec<MachineData>>,
    location: &u32,
    token: &String,
) -> Result<(), Box<dyn Error>> {
    let location_info = api::get_location_info(token, location)?;
    println!(
        "{}",
        format!("📍{}\n", location_info.data.name)
            .green()
            .underline()
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
    let mut table: Vec<HistoryTableRow> = Vec::new();

    for purchase in machines.data {
        let _type = purchase.serviceType;
        let description = {
            // Credit: https://stackoverflow.com/a/38406885/8653870
            let mut c = purchase.mutationDescription.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        };

        let amount = {
            let cost = purchase.mutationCents / 100;
            let currency = purchase.currency;
            // if cost > 0 {
            //     format!("{} {}", cost, currency).green()
            // } else {
            //     format!("{} {}", cost, currency).red()
            // }
            format!("{:3} {}", cost, currency)
        };

        let datetime = {
            let date = NaiveDateTime::from_timestamp(purchase.mutationTimestamp as i64, 0)
                + Duration::hours(2);
            let formatted_date = date.format("%d %b %Y %H:%M");
            formatted_date.to_string()
        };

        let emoji;
        match _type {
            Some(_type) => {
                emoji = match _type.as_str() {
                    "WASHING_MACHINE" => "🧺".to_string(),
                    _ => "💸".to_string(),
                };
            }
            _ => emoji = "💰".to_string(),
        }

        table.push(HistoryTableRow {
            _type: emoji,
            description,
            amount,
            date: datetime,
        });
    }

    let table = Table::new(table)
        .with(Style::rounded())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Columns::single(0)).with(Alignment::center()))
        .to_string();

    println!("{}", table);

    Ok(())
}

pub fn location_info(info: LocationInfo) -> Result<(), Box<dyn Error>> {
    let name = info.name;
    let id = info.externalId;
    let _type = info.locationTypeObject.name;

    println!("📍Name: {}", format!("{name}").green());
    println!("🔑ID: {}", format!("{id}").green());
    println!("🏢Type: {}", format!("{_type}").green());

    Ok(())
}
