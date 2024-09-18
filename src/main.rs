use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use std::io::{self, Write};
use std::env;
use std::process::exit;
use dotenv::dotenv;

fn main() {
    // Load API keys as env variables
    dotenv().ok();
    let healthchecks_api = match env::var("HEALTHCHECKS_API") {
        Ok(key) => key,
        Err(_e) => {
            println!("HealthChecks API Key not found. Exiting program...");
            exit(0);
        }
    };
    
    println!("HEALTHCHECKS_API: {healthchecks_api}");
    println!("Welcome to WATcloud!");

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        // ? What does unwrap do?
        let mut command = String::new();
        let client = Client::new();

        let mut header = HeaderMap::new();
        header.append("X-Api-Key", HeaderValue::from_static(""));

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        let command = command.trim();

        if command == "status" {
            let resp = client
                .get("https://healthchecks.io/api/v3/checks/")
                .headers(header)
                .send()
                .expect("Error in sending payload");

            let resp_text = resp.text().expect("Error in receiving info");
            println!("{resp_text}");
        } else if command == "exit" {
            exit(0);
        }
    }
}
