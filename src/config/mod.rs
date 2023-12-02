use crate::config::utils::save_config_file;
use crate::utils::input;
use crate::utils::out;
use reqwest::Error;
use serde::{Deserialize, Serialize};

mod defines;
mod utils;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    username: String,
    token: String,
    sort: defines::SORTING,
    protocol: defines::PROTOCOL,
    color: defines::COLOR,
    fancy: bool,
}

/// Checks if the prerequisites for tgh are installed.
/// If not, it will print an error and exit.
///
/// ### Arguments
/// * `args` - A vector of the command line arguments.
pub fn check_prerequisites(args: Vec<String>) {
    // Check if git is installed
    if !check_git() {
        out::print_error("Error: Git is not installed.\n");
        std::process::exit(1);
    }

    // Check for a GitHub token
    if !check_token() {
        if args.len() > 0 {
            if args[0] == "login" {
                return;
            }
        }

        out::print_error("Error: No GitHub token found.");
        println!("Please run `tgh login` to create one.\n");
        std::process::exit(1);
    }
}
fn check_git() -> bool {
    let mut command = std::process::Command::new("git");
    command.arg("--version");

    let output = command.output().unwrap();

    if output.status.success() {
        return true;
    }

    return false;
}
fn check_token() -> bool {
    if !utils::config_exists() || !utils::validate_config_file() {
        return false;
    }

    let config = utils::read_config();

    if config.token.len() == 0 {
        return false;
    }

    return true;
}

/// Authenticate user with GitHub.
/// @TODO: Split this function into smaller functions.
async fn authenticate() -> Result<String, Error> {
    use arboard::Clipboard;

    let client_id = "Iv1.d8c9cc38202b9305";
    let client = reqwest::Client::new();

    let mut text = client
        .post(format!(
            "https://github.com/login/device/code?client_id={}",
            client_id
        ))
        .send()
        .await?
        .text()
        .await?;

    text = text.replace('"', "").to_string();
    let text_split: Vec<String> = text
        .split("&")
        .map(|s| s.split("=").map(|s| s.to_string()).collect::<Vec<_>>()[1].to_string())
        .collect();

    let device_code = text_split[0].to_string();
    let expires_in = text_split[1].parse::<u64>().unwrap();
    let interval = text_split[2].parse::<u64>().unwrap();
    let user_code = text_split[3].to_string();
    let login_url = text_split[4].replace("%3A", ":").replace("%2F", "/");
    let grant_type = "urn:ietf:params:oauth:grant-type:device_code";

    println!("Please visit this URL to authenticate:\n{}", login_url);

    let clipboard = Clipboard::new();
    match clipboard {
        Ok(mut clipboard) => {
            clipboard.set_text(user_code.clone()).unwrap();
            println!(
                "Your user code has been copied to your clipboard. ({})",
                user_code
            )
        }
        Err(_) => {
            println!(
                "Error copying to clipboard, copy the code manually: {}",
                user_code
            );
        }
    }

    let params = [
        ("client_id", client_id),
        ("device_code", &device_code),
        ("grant_type", grant_type),
    ];

    let start_time = std::time::Instant::now();
    let token;

    loop {
        let res = client
            .post("https://github.com/login/oauth/access_token")
            .header(reqwest::header::ACCEPT, "application/json")
            .form(&params)
            .send()
            .await?;

        let mut res = res.text().await?;

        if res.contains("access_token") {
            res = res
                .replace("{", "")
                .replace("}", "")
                .replace('"', "")
                .replace("/", "");
            let res_split: Vec<String> = res
                .split(",")
                .map(|s| s.split(":").map(|s| s.to_string()).collect::<Vec<_>>()[1].to_string())
                .collect();

            token = res_split[0].to_string();

            break;
        }

        // Check if the authentication timed out
        if std::time::Instant::now()
            .duration_since(start_time)
            .as_secs()
            > (expires_in)
        {
            println!("Authentication timed out.");
            std::process::exit(1);
        }

        // Wait for the interval
        std::thread::sleep(std::time::Duration::from_secs(interval));
    }

    return Ok(token);
}

pub async fn login() {
    let token = authenticate().await;

    match token {
        Ok(token) => {
            update_token(token.clone());
        }
        Err(err) => {
            println!("{:?}", err);
            out::print_error("Error: Failed to authenticate.\n");
            std::process::exit(1);
        }
    }
}

/////////////////////////////////////////////////
/// Section of logic used for the config file ///
/////////////////////////////////////////////////

/// Loads the config file.
/// If the config file doesn't exist, it will create one.
/// If the config file is invalid, it will create a new one.
///
/// ### Returns
/// A Config struct.
pub fn load_config() -> Config {
    if utils::config_exists() {
        if utils::validate_config_file() {
            return utils::read_config();
        }
        out::print_error("Config file is invalid. Creating a new one...\n");
        return create_config();
    }

    out::print_error("Config file not found. Creating one...\n");
    return create_config();
}

fn create_config() -> Config {
    utils::handle_config_folder();

    let username = input::text("Enter your GitHub username: ", true);

    let config = Config {
        username,
        token: "".to_string(),
        sort: defines::SORTING::LastUpdated,
        protocol: defines::PROTOCOL::HTTPS,
        color: defines::COLOR::NORMAL,
        fancy: true,
    };

    save_config_file(config.clone());

    return config;
}

fn update_token(token: String) {
    let config = utils::read_config();

    let new_config = Config {
        username: config.username,
        token,
        sort: config.sort,
        protocol: config.protocol,
        color: config.color,
        fancy: config.fancy,
    };

    save_config_file(new_config);
}
