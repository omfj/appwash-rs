use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};

const DIR_NAME: &str = "appwash";
const CONFIG_FILE: &str = "config.toml";

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserConfig {
    pub account: Account,
    pub token: Token,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Token {
    pub secret: String,
    pub expires: u32,
}

impl Token {
    pub fn new(secret: String, expires: u32) -> Self {
        Self { secret, expires }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Account {
    pub email: String,
    pub password: String,
    pub location: u32,
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            account: Account {
                email: "<YOUR EMAIL>".to_string(),
                password: "<YOUR PASSWORD>".to_string(),
                location: 0,
            },
            token: Token {
                secret: "<TOKEN WILL BE AUTOMATICALLY GENERATED>".to_string(),
                expires: 0,
            },
        }
    }
}

impl UserConfig {
    /// Loads the user config from the XDG config directory.
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let mut data = String::new();

        let xdg_dirs = xdg::BaseDirectories::with_prefix(DIR_NAME)?;
        let config_path = xdg_dirs.place_config_file(CONFIG_FILE)?;

        let mut config_file = File::open(config_path)?;
        config_file.read_to_string(&mut data)?;

        let config: UserConfig = toml::from_str(&data)?;

        Ok(config)
    }

    /// Creates a default config file in the XDG config directory.
    pub fn _create() {
        let xdg_dirs = xdg::BaseDirectories::with_prefix(DIR_NAME)
            .expect("Failed to create XDG config directory");

        let config_path = xdg_dirs
            .place_config_file(CONFIG_FILE)
            .expect("Failed to create config file");

        let mut config_file = File::create(config_path).expect("Failed to create config file");
        let toml = toml::to_string(&UserConfig::default()).expect("Failed to serialize config");

        config_file
            .write_all(toml.as_bytes())
            .expect("Failed to write to config file");
    }

    pub fn set_token(&mut self, token: Token) {
        self.token = token;
    }
}
