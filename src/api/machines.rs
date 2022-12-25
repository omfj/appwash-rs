use std::{collections::HashMap, error::Error};

use serde_json::json;

use super::{
    miele::MieleCleint,
    models::{MachineData, ReserveStopResponse, Response},
};

/// Client to interact with the Miele API
impl MieleCleint {
    pub fn get_machines(&self) -> Result<Response<Vec<MachineData>>, Box<dyn Error>> {
        let url = format!(
            "https://www.involtum-services.com/api-rest/location/{}/connectorsv2",
            self.user.account.location
        );

        let mut headers = self.client_config.headers.clone();
        headers.insert("token", self.user.secret.token.parse()?);
        headers.insert("DNT", "1".parse()?);

        let mut json = HashMap::new();
        json.insert("serviceType", "WASHING_MACHINE");

        let response = self
            .client
            .post(&url)
            .json(&json)
            .headers(headers)
            .send()?
            .json::<Response<Vec<MachineData>>>()?;

        Ok(response)
    }

    pub fn stop_machine(&self, machine_id: u32) -> Result<ReserveStopResponse, Box<dyn Error>> {
        let url = format!(
            "https://www.involtum-services.com/api-rest/connector/{}/stop",
            machine_id
        );

        let mut headers = self.client_config.headers.clone();
        headers.insert("token", self.user.secret.token.parse()?);

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
        let url = format!(
            "https://www.involtum-services.com/api-rest/connector/{}/start",
            machine_id
        );

        let mut headers = self.client_config.headers.clone();
        headers.insert("token", self.user.secret.token.parse()?);

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
