use reqwest::header::HeaderMap;
use serde_json::Value;
use std::io::Write;
use dotenv;

fn prompt(prompt: &str) -> String {
    let mut line = String::new();
    let now = chrono::Utc::now();
    let hour_minute = now.format("%H:%M").to_string();

    println!("AppWash-CLI [{}]", hour_minute);
    print!("{} ", prompt);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");

    return line.trim().to_string()
}

fn commands(input: &str) {
    let command = input.split_whitespace().next().unwrap();
    // let args = input.split_whitespace().skip(1).collect::<Vec<&str>>();
    match command {
        "token" => { println!("Token is: {}", get_token().unwrap()) }
        "test" => { println!("Teesttest") }
        _ => { println!("Unknown command '{}'", command)}
    }
}

fn get_token() -> Result<String, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    // Assign .env
    let email = std::env::var("EMAIL").expect("EMAIL must be set");
    let password = std::env::var("PASSWORD").expect("PASSWORD must be set");

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
        .body("{\"email\":\"".to_string() + &email + "\",\"password\":\"" + &password + "\"}")
        .send()?;

    let resp = resp.text().unwrap();
    let resp_json: Value = serde_json::from_str(&resp)?;
    let token: String = resp_json["login"]["token"].to_string();
    // println!("Token is: {}", token);

    Ok(token)
}

fn main() {
    println!("Welcome to AppWash CLI\n");

    loop {
        let input = prompt(">>>");
        commands(&input);
        println!();
    }
}