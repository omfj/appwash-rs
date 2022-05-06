use ini::Ini;
use reqwest::header::HeaderMap;
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::LineWriter;
use std::io::Write;
use std::path::PathBuf;
use std::result::Result;

pub fn config_file_create(email: &str, password: &str) -> Result<(), Box<dyn Error>> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("appwash")?;
    let config_path = xdg_dirs.place_config_file("config")?;

    let config_file = File::create(config_path)?;

    let mut config_file = LineWriter::new(config_file);

    writeln!(config_file, "[DEFAULT]")?;
    writeln!(config_file, "EMAIL={}", email)?;
    writeln!(config_file, "PASSWORD={}", password)?;

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

pub fn load_config() -> Result<(String, String, String), Box<dyn Error>> {
    let config_path = xdg::BaseDirectories::with_prefix("appwash")
        .unwrap()
        .place_config_file("config")?;

    let config_path = PathBuf::from(config_path);

    let config = Ini::load_from_file(config_path)?;
    let section = config.section(Some("DEFAULT")).unwrap();

    let email = section.get("EMAIL").unwrap().to_string();
    let password = section.get("PASSWORD").unwrap().to_string();
    let token = get_token(&email, &password)?;

    Ok((email, password, token))
}

pub fn get_email() -> Result<String, Box<dyn Error>> {
    let config_path = xdg::BaseDirectories::with_prefix("appwash")
        .unwrap()
        .place_config_file("config")?;

    let config_path = PathBuf::from(config_path);

    let config = Ini::load_from_file(config_path)?;
    let section = config.section(Some("DEFAULT")).unwrap();

    Ok(section.get("EMAIL").unwrap().to_string())
}

// fn get_machines() -> Result<Value, Box<dyn Error>> {
//     let url = "https://www.involtum-services.com/api-rest/location/9944/connectorsv2";
//     let user_agent = "appwash-cli v0.1.0";
//     let token = account.token.to_owned();

//     let mut headers = HeaderMap::new();
//     headers.insert("token", token.parse().unwrap());
//     headers.insert("User-Agent", user_agent.parse().unwrap());
//     headers.insert("Referer", "https://appwash.com/".parse().unwrap());
//     headers.insert("language", "NO".parse().unwrap());
//     headers.insert("platform", "appWash".parse().unwrap());

//     let mut json: HashMap<String, String> = HashMap::new();
//     json.insert("serviceType".to_string(), "WASHING_MACHINE".to_string());

//     let client = reqwest::blocking::Client::new();
//     let resp = client.post(url).json(&json).headers(headers).send()?;

//     let resp = resp.text().unwrap();
//     let resp_json: Value = serde_json::from_str(&resp)?;

//     Ok(resp_json)
// }

pub fn get_token(email: &str, password: &str) -> Result<String, Box<dyn Error>> {
    // Basic request info
    let client = reqwest::blocking::Client::new();
    let url = "https://www.involtum-services.com/api-rest/login";
    let user_agent = "appwash-cli v0.1.0";

    // Headers
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("User-Agent", user_agent.parse().unwrap());
    headers.insert("language", "en".parse().unwrap());
    headers.insert("platform", "appWash".parse().unwrap());

    let resp = client
        .post(url)
        .headers(headers)
        .body("{\"email\":\"".to_string() + email + "\",\"password\":\"" + password + "\"}")
        .send()
        .unwrap();

    let resp = resp.text().unwrap();
    let resp_json: Value = serde_json::from_str(&resp).unwrap();
    let token: String = resp_json["login"]["token"].to_string().replace("\"", "");

    Ok(token)
}
