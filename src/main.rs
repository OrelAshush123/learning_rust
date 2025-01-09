// I LEARN RUST FROM https://www.youtube.com/watch?v=OX9HJsJUDxA&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=2
use std::io::{self, Write};
use reqwest::blocking::Client;
use serde_json::Value;

//PROJECT FOR GIT HUB
fn main() {
    println!("Hello, GIT!");

    let mut username = String::new();
    let mut token = String::new();

    print!("Enter your GitHub username: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();

    print!("Enter your GitHub personal access token: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut token).unwrap();
    let token = token.trim();

    let client = Client::new();
    let url = format!("https://api.github.com/user/repos");
    let response = client
        .get(&url)
        .basic_auth(username, Some(token))
        .header("User-Agent", "request")
        .send()
        .unwrap();

    if response.status().is_success() {
        let repos: Value = response.json().unwrap();
        for repo in repos.as_array().unwrap() {
            println!("{}", repo["name"].as_str().unwrap());
        }
    } else {
        println!("Failed to fetch repositories: {}", response.status());
    }
}
