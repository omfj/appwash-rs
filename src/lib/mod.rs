#![allow(non_snake_case)] // For JSON deserialization
pub mod models;
pub mod pretty;

use ini::Ini;
use reqwest::header::HeaderMap;
use serde_json::json;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::LineWriter;
use std::io::Write;
use std::path::PathBuf;
use std::result::Result;

use self::models::LocationInfo;
use self::models::{
    BalanceResponse, History, LoginResponse, MachineData, ReserveStopResponse, Response,
};

const USER_AGENT: &str = "appwash-rs v1.0";

pub fn config_file_create(email: &str, password: &str) -> Result<(), Box<dyn Error>> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("appwash")?;
    let config_path = xdg_dirs.place_config_file("config")?;

    let config_file = File::create(config_path)?;

    let mut config_file = LineWriter::new(config_file);

    writeln!(config_file, "[ACCOUNT]")?;
    writeln!(config_file, "EMAIL={}", email)?;
    writeln!(config_file, "PASSWORD={}", password)?;
    writeln!(config_file, "LOCATION={}", 0)?;

    Ok(())
}

pub fn change_location(location: u32) -> Result<(), Box<dyn Error>> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("appwash")?;
    let config_path = xdg_dirs.place_config_file("config")?;

    let mut config = Ini::load_from_file(config_path.clone())?;

    config
        .with_section(Some("ACCOUNT"))
        .set("LOCATION", &location.to_string());

    config.write_to_file(config_path)?;

    Ok(())
}

pub fn config_file_exists() -> bool {
    let config_path = xdg::BaseDirectories::with_prefix("appwash")
        .unwrap()
        .place_config_file("config")
        .unwrap()
        .exists();

    config_path
}

pub fn load_config() -> Result<(String, String, u32, String), Box<dyn Error>> {
    let config_path = xdg::BaseDirectories::with_prefix("appwash")
        .unwrap()
        .place_config_file("config")?;

    let config_path = PathBuf::from(config_path);

    let config = Ini::load_from_file(config_path)?;
    let section = config.section(Some("ACCOUNT")).unwrap();

    let email = section
        .get("EMAIL")
        .expect(
            "Failed to load config file. Make sure you provided username, password and location.",
        )
        .to_string();
    let password = section
        .get("PASSWORD")
        .expect(
            "Failed to load config file. Make sure you provided username, password and location.",
        )
        .to_string();
    let location = section
        .get("LOCATION")
        .expect(
            "Failed to load config file. Make sure you provided username, password and location.",
        )
        .parse::<u32>()
        .expect("Failed to parse location.");
    let token = get_token(&email, &password)?;

    Ok((email, password, location, token))
}

pub fn get_machines(
    token: &String,
    location: &u32,
) -> Result<Response<Vec<MachineData>>, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let url =
        format!("https://www.involtum-services.com/api-rest/location/{location}/connectorsv2");

    let mut headers = get_headers()?;
    headers.insert("token", token.parse().unwrap());
    headers.insert("DNT", "1".parse().unwrap());

    let mut json = HashMap::new();
    json.insert("serviceType", "WASHING_MACHINE");

    let machines = client
        .post(&url)
        .json(&json)
        .headers(headers)
        .send()?
        .json::<Response<Vec<MachineData>>>()?;

    Ok(machines)
}

pub fn get_location_info(
    token: &String,
    location: &u32,
) -> Result<Response<LocationInfo>, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let url = format!("https://www.involtum-services.com/api-rest/locations/split/{location}");

    let mut headers = get_headers()?;
    headers.insert("token", token.parse().unwrap());

    let location_info = client
        .get(&url)
        .headers(headers)
        .send()?
        .json::<Response<LocationInfo>>()?;

    Ok(location_info)
}

pub fn get_balance(token: &String) -> Result<(u32, String), Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let url = "https://www.involtum-services.com/api-rest/account/getprepaid";

    let mut headers = get_headers()?;
    headers.insert("token", token.parse().unwrap());

    let resp = client
        .get(url)
        .headers(headers)
        .send()?
        .json::<BalanceResponse>()?;

    let balance = resp.balanceCents / 100;
    let currency = resp.currency;

    Ok((balance, currency))
}

pub fn stop_machine(
    token: &String,
    machine_id: &u32,
) -> Result<ReserveStopResponse, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let url = format!("https://www.involtum-services.com/api-rest/connector/{machine_id}/stop");

    let mut headers = get_headers()?;
    headers.insert("token", token.parse().unwrap());

    let resp = client
        .post(&url)
        .headers(headers)
        .send()?
        .json::<ReserveStopResponse>()?;

    Ok(resp)
}

pub fn get_history(token: &String) -> Result<Response<Vec<History>>, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let url = "https://www.involtum-services.com/api-rest/account/getprepaidmutations";

    let mut headers = get_headers()?;
    headers.insert("token", token.parse().unwrap());

    let resp = client
        .get(url)
        .headers(headers)
        .send()?
        .json::<Response<Vec<History>>>()?;

    Ok(resp)
}

pub fn reserve_machine(
    token: &String,
    machine_id: &u32,
) -> Result<ReserveStopResponse, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let url = format!(
        "https://www.involtum-services.com/api-rest/connector/{}/start",
        machine_id
    );

    let mut headers = get_headers()?;
    headers.insert("token", token.parse().unwrap());

    let resp = client
        .post(&url)
        .headers(headers)
        .send()?
        .json::<ReserveStopResponse>()?;

    Ok(resp)
}

fn get_headers() -> Result<HeaderMap, Box<dyn Error>> {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("User-Agent", USER_AGENT.parse().unwrap());
    headers.insert("language", "NO".parse().unwrap());
    headers.insert("platform", "appWash".parse().unwrap());
    headers.insert("Referer", "https://appwash.com/".parse().unwrap());

    Ok(headers)
}

pub fn get_token(email: &str, password: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let url = "https://www.involtum-services.com/api-rest/login";

    let headers = get_headers()?;

    let token = client
        .post(url)
        .headers(headers)
        .body(json!({ "email": email, "password": password }).to_string())
        .send()?
        .json::<LoginResponse>()?
        .login
        .token;

    Ok(token)
}
