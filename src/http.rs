use std::{collections::HashMap, error::Error};

use reqwest::{blocking::Client, header::HeaderMap};
use serde_json::json;

use crate::{
    models::{
        BalanceResponse, History, LocationInfo, LoginResponse, MachineData, ReserveStopResponse,
        Response,
    },
    user::UserConfig,
};

const BASE_URL: &str = "https://www.involtum-services.com/api-rest";

pub fn create_default_client() -> Client {
    let user_agent = "appwash-cli v1.0.0".to_string();

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("User-Agent", user_agent.parse().unwrap());
    headers.insert("language", "EN".parse().unwrap());
    headers.insert("platform", "appWash".parse().unwrap());
    headers.insert("Referer", "https://appwash.com/".parse().unwrap());

    let client = Client::builder()
        .default_headers(headers)
        .user_agent(user_agent)
        .build()
        .expect("Failed to build http client");

    client
}

pub struct AppwashClient {
    pub client: Client,
    pub user: UserConfig,
}

impl AppwashClient {
    pub fn new(user: UserConfig) -> Result<Self, Box<dyn Error>> {
        let client = create_default_client();
        let user = user;

        Ok(Self { client, user })
    }

    pub fn get_balance(&self) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/account/getprepaid", BASE_URL);

        let mut headers = HeaderMap::new();
        headers.insert("token", self.user.token.secret.parse()?);

        let response = self.client.get(url).headers(headers).send()?;

        let json = response.json::<BalanceResponse>()?;

        let balance = json.balance_cents / 100;
        let currency = json.currency;

        Ok(format!("{} {}", balance, currency))
    }

    pub fn get_location_info(&self) -> Result<Response<LocationInfo>, Box<dyn Error>> {
        let url = format!(
            "{}/locations/split/{}",
            BASE_URL, self.user.account.location
        );

        let response = self
            .client
            .get(url)
            .send()?
            .json::<Response<LocationInfo>>()?;

        Ok(response)
    }

    pub fn get_history(&self) -> Result<Response<Vec<History>>, Box<dyn Error>> {
        let url = format!("{}/account/getprepaidmutations", BASE_URL);

        let response = self
            .client
            .get(url)
            .send()?
            .json::<Response<Vec<History>>>()?;

        Ok(response)
    }

    pub fn login(&self) -> Result<LoginResponse, Box<dyn Error>> {
        let url = format!("{}/login", BASE_URL);

        let mut map = HashMap::new();
        map.insert("email".to_string(), self.user.account.email.clone());
        map.insert("password".to_string(), self.user.account.password.clone());

        let response = self
            .client
            .post(url)
            .json(&map)
            .send()?
            .json::<LoginResponse>()?;

        Ok(response)
    }

    pub fn get_machines(&self) -> Result<Response<Vec<MachineData>>, Box<dyn Error>> {
        let url = format!(
            "{}/location/{}/connectorsv2",
            BASE_URL, self.user.account.location
        );

        let mut headers = HeaderMap::new();
        headers.insert("token", self.user.token.secret.parse()?);
        headers.insert("DNT", "1".parse()?);

        let mut map = HashMap::new();
        map.insert("serviceType", "WASHING_MACHINE");

        let response = self
            .client
            .post(&url)
            .json(&map)
            .headers(headers)
            .send()?
            .json::<Response<Vec<MachineData>>>()?;

        Ok(response)
    }

    pub fn stop_machine(&self, machine_id: u32) -> Result<ReserveStopResponse, Box<dyn Error>> {
        let url = format!("{}/connector/{}/stop", BASE_URL, machine_id);

        let mut headers = HeaderMap::new();
        headers.insert("token", self.user.token.secret.parse()?);

        let map = json!({
            "objectId": null,
            "objectLength": null,
            "objectName": null,
            "nrOfPersons": null,
            "freeFormQuestionValue": null,
            "comment": null,
            "sourceChannel": "WEBSITE"
        });

        let response = self
            .client
            .post(&url)
            .headers(headers)
            .json(&map)
            .send()?
            .json::<ReserveStopResponse>()?;

        Ok(response)
    }

    pub fn reserve_machine(&self, machine_id: &u32) -> Result<ReserveStopResponse, Box<dyn Error>> {
        let url = format!("{}/connector/{}/start", BASE_URL, machine_id);

        let mut headers = HeaderMap::new();
        headers.insert("token", self.user.token.secret.parse()?);

        let map = json!({
            "objectId": null,
            "objectLength": null,
            "objectName": null,
            "nrOfPersons": null,
            "freeFormQuestionValue": null,
            "comment": null,
            "sourceChannel": "WEBSITE"
        });

        let response = self
            .client
            .post(&url)
            .headers(headers)
            .json(&map)
            .send()?
            .json::<ReserveStopResponse>()?;

        Ok(response)
    }
}
