use super::models::LocationInfo;
use super::models::{
    BalanceResponse, History, LoginResponse, MachineData, ReserveStopResponse, Response,
};
use reqwest::header::HeaderMap;
use serde_json::json;
use std::collections::HashMap;
use std::error::Error;

const USER_AGENT: &str = "appwash-rs v1.0";

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

    let map = json!({
        "objectId": null,
        "objectLength": null,
        "objectName": null,
        "nrOfPersons": null,
        "freeFormQuestionValue": null,
        "comment": null,
        "sourceChannel": "WEBSITE"
    });

    let resp = client
        .post(&url)
        .headers(headers)
        .json(&map)
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

    let map = json!({
        "objectId": null,
        "objectLength": null,
        "objectName": null,
        "nrOfPersons": null,
        "freeFormQuestionValue": null,
        "comment": null,
        "sourceChannel": "WEBSITE"
    });

    let resp = client
        .post(&url)
        .headers(headers)
        .json(&map)
        .send()?
        .json::<ReserveStopResponse>()?;

    Ok(resp)
}

fn get_headers() -> Result<HeaderMap, Box<dyn Error>> {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("User-Agent", USER_AGENT.parse().unwrap());
    headers.insert("language", "EN".parse().unwrap());
    headers.insert("platform", "appWash".parse().unwrap());
    headers.insert("Referer", "https://appwash.com/".parse().unwrap());

    Ok(headers)
}

pub fn get_token(email: &str, password: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let url = "https://www.involtum-services.com/api-rest/login";

    let headers = get_headers()?;

    let resp = client
        .post(url)
        .headers(headers)
        .body(json!({ "email": email, "password": password }).to_string())
        .send()?
        .json::<LoginResponse>()?;

    let token = resp.login.token;

    Ok(token)
}
