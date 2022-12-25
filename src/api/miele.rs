use reqwest::header::HeaderMap;

use crate::user::UserConfig;

pub struct ClientConfig {
    pub user_agent: String,
    pub headers: HeaderMap,
}

impl Default for ClientConfig {
    fn default() -> Self {
        let user_agent = "appwash-cli v1.0.0".to_string();

        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("User-Agent", user_agent.parse().unwrap());
        headers.insert("language", "EN".parse().unwrap());
        headers.insert("platform", "appWash".parse().unwrap());
        headers.insert("Referer", "https://appwash.com/".parse().unwrap());

        Self {
            headers,
            user_agent,
        }
    }
}

pub struct MieleCleint {
    pub user: UserConfig,
    pub client: reqwest::blocking::Client,
    pub client_config: ClientConfig,
}

impl MieleCleint {
    pub fn new(user: UserConfig) -> Self {
        let client = reqwest::blocking::Client::new();
        let client_config = ClientConfig::default();

        Self {
            user,
            client,
            client_config,
        }
    }
}
