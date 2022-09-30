use super::models::LocationInfo;
use super::models::{BalanceResponse, History, MachineData, ReserveStopResponse, Response};
use crate::User;
use reqwest::header::HeaderMap;
use serde_json::json;
use std::collections::HashMap;
use std::error::Error;

const USER_AGENT: &str = "appwash-cli v1.0.0";

pub fn get_machines(user: &User) -> Result<Response<Vec<MachineData>>, Box<dyn Error>> {
    let location = user.location;
    let token = &user.token;

    let client = reqwest::blocking::Client::new();
    let url = format!(
        "https://www.involtum-services.com/api-rest/location/{}/connectorsv2",
        location
    );

    let mut headers = get_headers();
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

pub fn get_location_info(user: &User) -> Result<Response<LocationInfo>, Box<dyn Error>> {
    let location = user.location;
    let token = &user.token;

    let client = reqwest::blocking::Client::new();
    let url = format!(
        "https://www.involtum-services.com/api-rest/locations/split/{}",
        location
    );

    let mut headers = get_headers();
    headers.insert("token", token.parse().unwrap());

    let location_info = client
        .get(url)
        .headers(headers)
        .send()?
        .json::<Response<LocationInfo>>()?;

    Ok(location_info)
}

pub fn get_balance(user: &User) -> Result<(u32, String), Box<dyn Error>> {
    let token = &user.token;

    let client = reqwest::blocking::Client::new();
    let url = "https://www.involtum-services.com/api-rest/account/getprepaid";

    let mut headers = get_headers();
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
    mut user: User,
    machine_id: u32,
) -> Result<ReserveStopResponse, Box<dyn Error>> {
    let token = user.generate_token()?;

    let client = reqwest::blocking::Client::new();
    let url = format!("https://www.involtum-services.com/api-rest/connector/{machine_id}/stop");

    let mut headers = get_headers();
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

pub fn get_history(user: &User) -> Result<Response<Vec<History>>, Box<dyn Error>> {
    let token = &user.token;

    let client = reqwest::blocking::Client::new();
    let url = "https://www.involtum-services.com/api-rest/account/getprepaidmutations";

    let mut headers = get_headers();
    headers.insert("token", token.parse().unwrap());

    let resp = client
        .get(url)
        .headers(headers)
        .send()?
        .json::<Response<Vec<History>>>()?;

    Ok(resp)
}

pub fn reserve_machine(
    user: &User,
    machine_id: u32,
) -> Result<ReserveStopResponse, Box<dyn Error>> {
    let token = &user.token;

    let client = reqwest::blocking::Client::new();
    let url = format!(
        "https://www.involtum-services.com/api-rest/connector/{}/start",
        machine_id
    );

    let mut headers = get_headers();
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

pub fn get_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("User-Agent", USER_AGENT.parse().unwrap());
    headers.insert("language", "EN".parse().unwrap());
    headers.insert("platform", "appWash".parse().unwrap());
    headers.insert("Referer", "https://appwash.com/".parse().unwrap());

    headers
}
