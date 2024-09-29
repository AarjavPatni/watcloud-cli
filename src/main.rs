use dotenv::dotenv;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::from_str;
use std::env;
use std::io::{self, Write};
use std::process::exit;
extern crate regex;
use regex::Regex;

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
        let mut command = String::new();
        let client = Client::new();

        let mut header = HeaderMap::new();
        header.append(
            "X-Api-Key",
            HeaderValue::from_str(&healthchecks_api).unwrap(),
        );

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
            // println!("{resp_text}");
            let mut resp_json: serde_json::Value = from_str(&resp_text).unwrap();
            // println!("{resp_json}");

            let mut resp_arr: Vec<serde_json::Value> =
                resp_json["checks"].as_array_mut().unwrap().clone();

            // * TODO: For each check, add a new key called "host" which holds the value from "host=" from the "tags" key

            // * TODO: For each check, add a new key called "check" which holds the value from "check=" from the "tags" key

            // * TODO: Sort the checks using "host" key

            // TODO: Print the sorted checks grouped by host

            // TODO: Print the sorted checks grouped by check

            // Adds host key to each check
            let host_regex = Regex::new(r"host=[^\s]+").unwrap();

            for i in resp_arr.iter_mut() {
                let instance_host;
                match host_regex.find(i["tags"].as_str().unwrap()) {
                    Some(mat) => {
                        instance_host = mat.as_str().to_string().replace("host=", "");
                    }
                    None => {
                        println!("No host found");
                        continue;
                    }
                }
                i.as_object_mut().unwrap().insert(
                    "host".to_string(),
                    serde_json::json!(format!("{}", instance_host)),
                );
            }

            // Adds check key to each check
            let check_regex = Regex::new(r"check=[^\s]+").unwrap();
            for i in resp_arr.iter_mut() {
                let instance_check;
                match check_regex.find(i["tags"].as_str().unwrap()) {
                    Some(mat) => {
                        instance_check = mat.as_str().to_string().replace("check=", "");
                    }
                    None => {
                        println!("No check found");
                        continue;
                    }
                }
                i.as_object_mut().unwrap().insert(
                    "check".to_string(),
                    serde_json::json!(format!("{}", instance_check)),
                );
            }

            // Sort checks by host
            resp_arr.sort_by_key(|k| k.get("host").unwrap().as_str().unwrap().to_string());

            // Print checks by host
            for i in resp_arr {
                println!("{i}");
            }
        } else if command == "exit" {
            exit(0);
        }
    }
}
