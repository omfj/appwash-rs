extern crate ini;
use ini::Ini;
use reqwest::header::HeaderMap;
use serde_json::Value;
use std::io::Write;
use std::process::exit;
use std::fs::File;
use std::io::LineWriter;

struct Account {
    email: String,
    password: String,
    token: String,
}

fn create_config() -> Result<(), Box<dyn std::error::Error>> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("appwash").unwrap();
    let config_path = xdg_dirs.place_config_file("config")
                                        .expect("Cannot create config file.");

    let config_file = File::create(config_path)?;
    let mut config_file = LineWriter::new(config_file);
    config_file.write_all(b"[DEFAULT]\nEMAIL=<appwash email>\nPASSWORD=<appwash password>")?;

    Ok(())
}

fn get_user_info() -> Result<(String, String, String), Box<dyn std::error::Error>> {
    let (email, password, token): (String, String, String);

    let xdg_dirs = xdg::BaseDirectories::with_prefix("appwash").unwrap();
    let config_path = xdg_dirs.find_config_file("config").expect("Could not find config file.");

    let config = Ini::load_from_file(config_path).unwrap();
    let section = config.section(Some("Account")).unwrap();

    email = section.get("EMAIL").unwrap().trim().to_string();
    password = section.get("PASSWORD").unwrap().trim().to_string();
    token = get_token(&email, &password);

    Ok((email, password, token))
}

fn get_machines(account: &Account) -> Result<Value, Box<dyn std::error::Error>> {
    let url = "https://www.involtum-services.com/api-rest/location/9944/connectorsv2";
    let user_agent = "appwash-cli v0.1.0";
    let client = reqwest::blocking::Client::new();
    let token = account.token.as_str();

    let mut headers = HeaderMap::new();
    headers.insert("user-agent", user_agent.parse().unwrap());
    headers.insert("referer",    "https://appwash.com/".parse().unwrap());
    headers.insert("token",      token.parse().unwrap());
    headers.insert("language",   "NO".parse().unwrap());
    headers.insert("platform",   "appWash".parse().unwrap());

    let resp = client
                .post(url)
                .headers(headers)
                .send()?;

    let resp = resp.text().unwrap();
    let resp_json: Value = serde_json::from_str(&resp)?;

    Ok(resp_json)
}

fn get_token(email: &String, password: &String) -> String {
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
    let token: String = resp_json["login"]["token"].to_string();

    token
}

fn prompt(prompt: &str) -> String {
    let mut line = String::new();
    let now = chrono::Utc::now();
    let hour_minute = now.format("%H:%M").to_string();

    println!("AppWash-CLI [{}]", hour_minute);
    print!("{} ", prompt);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");

    line.trim().to_string()
}

fn commands(input: &str, account: &Account) {
    let command = input.split_whitespace().next().unwrap();
    // let args = input.split_whitespace().skip(1).collect::<Vec<&str>>();

    match command {
        "exit" | "e" => exit(1),
        "whoami" => {
            println!("You are...");
            println!("Email: {}", account.email);
            println!("Password: {}", account.password);
            println!("Token: {}", account.token);
        }
        "list" => println!("{:#?}", get_machines(account).unwrap()),
        "create-config" => {
            match create_config() {
                Ok(()) => { println!("Successfully created config file.") }
                Err(_) => { println!("An error has occured creating the config file.") },
            }
        }
        _ => println!("Unknown command '{}'.", command),
    }
}

fn main() {
    println!("Welcome to AppWash CLI\n");

    let (mut email, mut password, mut token): (String, String, String) = (String::new(), String::new(), String::new());

    //(email, password, token) = get_user_info().unwrap();

    match get_user_info() {
        Ok(acc) => (email, password, token) = acc,
        Err(_) => println!("Could not load account"),
    }

    let account: Account = Account {
        email: email,
        password: password,
        token: token,
    };

    loop {
        let input = prompt(">>>");
        commands(&input, &account);
        println!();
    }
}