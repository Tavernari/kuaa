use ansi_term::Colour::{Red, Yellow};
use ansi_term::Style;
use clap::{Parser, Subcommand};
use dotenv::dotenv;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, Error as ReqwestError}; // Rename Reqwest Error
use serde_json::json;
use std::env;
use std::fs::{write, OpenOptions};
use std::io::{Error as IoError, Write}; // Rename IO Error
use std::process::Command;
use tokio::runtime::Runtime;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    result: ResultContent,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResultContent {
    content: String,
    usage: Usage,
}

#[derive(Debug, Serialize, Deserialize)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct KTokensBalance {
    balance: u128,
}

#[derive(Debug, Serialize, Deserialize)]
struct KTokensBalanceData {
    data: KTokensBalance,
}

const KUAA_API_KEY_ENV: &str = "KUAA_API_KEY"; // Constant for the environment variable name

fn get_git_diff_staged() -> Result<String, std::io::Error> {
    let output = Command::new("git").args(["diff", "--cached"]).output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to get git diff",
        ))
    }
}

async fn fetch_balance(api_key: &str) -> Result<(), ReqwestError> {
    let client = Client::new();
    // let url = "https://kuaa.tools/api/ktokens/balance";
    let url = "http://localhost:5003/api/ktokens/balance"; // Use this URL for local testing

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", api_key).parse().unwrap(),
    );
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let response = client.get(url).headers(headers).send().await?;

    if response.status().is_success() {
        let response_json: KTokensBalanceData = response.json().await?;

        println!(
            "{}: {}",
            Style::new().bold().paint("K-Tokens Balance\n"),
            Style::new()
                .bold()
                .italic()
                .paint(response_json.data.balance.to_string())
        );
    } else {
        println!(
            "{} {}",
            Red.bold().paint("Failed to send data. Status:"),
            response.status()
        );
    }

    Ok(())
}

async fn send_git_diff(api_key: &str, diff: String, comments: String) -> Result<(), ReqwestError> {
    let client = Client::new();
    // let url = "https://kuaa.tools/api/prompt";
    let url = "http://localhost:5003/api/prompt"; // Use this URL for local testing

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", api_key).parse().unwrap(),
    );
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let body = json!({
        "data":{
            "promptModel":{
                "type": "git-commit-message",
                "data": {
                    "diff": diff,
                    "comments": comments
                }
            }
        }
    });

    let response = client.post(url).headers(headers).json(&body).send().await?;

    if response.status().is_success() {
        let response_json: ApiResponse = response.json().await?;

        println!("{}", Style::new().bold().paint("### Git Commit Message\n"));
        println!(
            "{}",
            Style::new()
                .bold()
                .italic()
                .paint(&response_json.result.content)
        );
        println!("{}", Style::new().bold().paint("\n### Usage Summary\n"));
        println!(
            "- Prompt Tokens: {}",
            Yellow.paint(response_json.result.usage.prompt_tokens.to_string())
        );
        println!(
            "- Completion Tokens: {}",
            Yellow.paint(response_json.result.usage.completion_tokens.to_string())
        );
        println!(
            "- Total Tokens: {}",
            Yellow.paint(response_json.result.usage.total_tokens.to_string())
        );
    } else {
        println!(
            "{} {}",
            Red.bold().paint("Failed to send data. Status:"),
            response.status()
        );
    }

    Ok(())
}

fn save_api_key_to_env_file(key: &str) -> Result<(), IoError> {
    let env_file_path = ".env";

    // Check if the .env file already exists and contains the API_KEY variable
    let env_contents = std::fs::read_to_string(env_file_path).unwrap_or_default();
    let contains_api_key = env_contents
        .lines()
        .any(|line| line.starts_with(&format!("{}=", KUAA_API_KEY_ENV)));

    if contains_api_key {
        // If API_KEY is already defined, we'll replace its value
        let new_contents = env_contents
            .lines()
            .map(|line| {
                if line.starts_with(&format!("{}=", KUAA_API_KEY_ENV)) {
                    format!("{}={}", KUAA_API_KEY_ENV, key)
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");

        write(env_file_path, new_contents)?;
    } else {
        // If the file doesn't exist or API_KEY is not defined, append it
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(env_file_path)?;

        writeln!(file, "{}={}", KUAA_API_KEY_ENV, key)?;
    }

    Ok(())
}

/// A simple CLI tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Kuaa {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum ConfigCommand {
    /// Sets the API key
    ApiKey {
        #[arg(name = "api-key")]
        key: String,
    },
}

#[derive(Subcommand, Debug, Clone)]
enum GenCommand {
    /// Generate Git Commit Message
    GitCommitMessage {
        #[arg(name = "git-commit-message")]
        git_commit_message: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Configure settings
    Config {
        #[command(subcommand)]
        config_command: ConfigCommand,
    },
    /// Generate git commit message
    Gen {
        #[command(subcommand)]
        gen_command: GenCommand,
    },
    /// Fetch K-Tokens balance
    Balance {},
}

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        dotenv().ok(); // Load .env file contents into environment variables

        let cli = Kuaa::parse();

        match cli.command {
            Commands::Config { config_command } => match config_command {
                ConfigCommand::ApiKey { key } => {
                    println!("Setting API key to: {}", key);
                    if let Err(e) = save_api_key_to_env_file(&key) {
                        eprintln!("Failed to save API key to .env file: {}", e);
                    }
                }
            },
            Commands::Gen { gen_command } => match gen_command {
                GenCommand::GitCommitMessage { git_commit_message } => {
                    let diff = get_git_diff_staged().unwrap_or_else(|_| "".to_string());
                    let comments = git_commit_message.unwrap_or_else(|| "".to_string());

                    match env::var(KUAA_API_KEY_ENV) {
                        Ok(api_key) => {
                            if let Err(e) = send_git_diff(&api_key, diff, comments).await {
                                eprintln!("Failed to send git diff: {}", e);
                            }
                        }
                        Err(_) => println!("{} environment variable is not set.", KUAA_API_KEY_ENV),
                    }
                }
            },
            Commands::Balance {} => match env::var(KUAA_API_KEY_ENV) {
                Ok(api_key) => {
                    if let Err(e) = fetch_balance(&api_key).await {
                        eprintln!("Failed to send git diff: {}", e);
                    }
                }
                Err(_) => println!("{} environment variable is not set.", KUAA_API_KEY_ENV),
            },
        }
    });
}
