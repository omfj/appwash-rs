use std::error::Error;

use super::miele::MieleCleint;
use super::models::{BalanceResponse, History, LocationInfo, LoginResponse, Response};

impl MieleCleint {
    pub fn get_balance(&self) -> Result<String, Box<dyn Error>> {
        let url = "https://www.involtum-services.com/api-rest/account/getprepaid";

        let mut headers = self.client_config.headers.clone();
        headers.insert("token", self.user.secret.token.parse()?);

        let response = self
            .client
            .get(url)
            .headers(headers)
            .send()?
            .json::<BalanceResponse>()?;

        let balance = response.balance_cents / 100;
        let currency = response.currency;

        Ok(format!("{} {}", balance, currency))
    }

    pub fn get_location_info(&self) -> Result<Response<LocationInfo>, Box<dyn Error>> {
        let url = format!(
            "https://www.involtum-services.com/api-rest/locations/split/{}",
            self.user.account.location
        );

        let mut headers = self.client_config.headers.clone();
        headers.insert("token", self.user.secret.token.parse()?);

        let response = self
            .client
            .get(url)
            .headers(headers)
            .send()?
            .json::<Response<LocationInfo>>()?;

        Ok(response)
    }

    pub fn get_history(&self) -> Result<Response<Vec<History>>, Box<dyn Error>> {
        let url = "https://www.involtum-services.com/api-rest/account/getprepaidmutations";

        let mut headers = self.client_config.headers.clone();
        headers.insert("token", self.user.secret.token.parse()?);

        let response = self
            .client
            .get(url)
            .headers(headers)
            .send()?
            .json::<Response<Vec<History>>>()?;

        Ok(response)
    }

    pub fn login(&self) -> Result<Response<LoginResponse>, Box<dyn Error>> {
        let url = "https://www.involtum-services.com/api-rest/account/login";

        let mut headers = self.client_config.headers.clone();
        headers.insert("email", self.user.account.email.parse().unwrap());
        headers.insert("password", self.user.account.password.parse().unwrap());

        let response = self
            .client
            .post(url)
            .headers(headers)
            .send()?
            .json::<Response<LoginResponse>>()?;

        Ok(response)
    }
}
