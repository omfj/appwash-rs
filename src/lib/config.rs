use ini::Ini;

use std::error::Error;
use std::fs::File;
use std::io::LineWriter;
use std::io::Write;
use std::path::PathBuf;
use std::result::Result;

use super::api;

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
    let token = api::get_token(&email, &password)?;

    Ok((email, password, location, token))
}
