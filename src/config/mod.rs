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
pub fn check_prerequisites(mode: String) {
    // Check if git is installed
    if !check_git() {
        out::print_error("Error: Git is not installed.\n");
        std::process::exit(1);
    }

    // Check for a GitHub token
    if !check_token() {
        if mode.len() > 0 {
            if mode == "login" {
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

            out::print_success("Successfully authenticated.\n");
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

    let username = ask_username();
    let sort = ask_sort();
    let protocol = ask_protocol();
    let color = ask_color();
    let fancy = ask_fancy();

    let config = Config {
        username,
        token: "".to_string(),
        sort,
        protocol,
        color,
        fancy,
    };

    utils::save_config_file(config.clone());

    out::print_success("Successfully created config file.\n");

    return config;
}

fn ask_username() -> String {
    use inquire::{required, Text};

    let username = Text::new("Enter your GitHub username:")
        .with_validator(required!("Username is required."))
        .prompt();

    return username.unwrap();
}
fn ask_sort() -> defines::SORTING {
    use inquire::Select;

    let option = Select::new(
        "Select a sorting method:",
        vec!["Last Updated", "Alphabetical"],
    )
    .with_page_size(2)
    .prompt()
    .unwrap();

    return match option {
        "Last Updated" => defines::SORTING::LastUpdated,
        "Alphabetical" => defines::SORTING::Alphabetical,
        _ => {
            out::print_error("Invalid input.\n");
            ask_sort()
        }
    };
}
fn ask_protocol() -> defines::PROTOCOL {
    use inquire::Select;

    let option = Select::new("Select a protocol:", vec!["HTTPS", "SSH"])
        .with_page_size(2)
        .prompt()
        .unwrap();

    return match option {
        "HTTPS" => defines::PROTOCOL::HTTPS,
        "SSH" => defines::PROTOCOL::SSH,
        _ => {
            out::print_error("Invalid input.\n");
            ask_protocol()
        }
    };
}

fn ask_color() -> defines::COLOR {
    use inquire::Select;

    let option = Select::new(
        "Select a color for the output:",
        vec![
            "default", "red", "green", "yellow", "blue", "magenta", "cyan", "white", "gray",
        ],
    )
    .with_page_size(5)
    .prompt();

    match option {
        Ok(option) => {
            return match option {
                "default" => defines::COLOR::NORMAL,
                "red" => defines::COLOR::RED,
                "green" => defines::COLOR::GREEN,
                "yellow" => defines::COLOR::YELLOW,
                "blue" => defines::COLOR::BLUE,
                "magenta" => defines::COLOR::MAGENTA,
                "cyan" => defines::COLOR::CYAN,
                "white" => defines::COLOR::WHITE,
                "gray" => defines::COLOR::GRAY,
                _ => {
                    out::print_error("Invalid input.\n");
                    ask_color()
                }
            };
        }
        Err(_) => {
            out::print_error("Invalid input.\n");
            return ask_color();
        }
    }
}
fn ask_fancy() -> bool {
    use inquire::Confirm;

    let option = Confirm::new("Enable fancy commits?")
        .with_default(true)
        .prompt();

    match option {
        Ok(option) => return option,
        Err(_) => {
            out::print_error("Invalid input.\n");
            return ask_fancy();
        }
    }
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

    utils::save_config_file(new_config);
}
fn update_username(username: String) {
    let config = utils::read_config();

    let new_config = Config {
        username,
        token: config.token,
        sort: config.sort,
        protocol: config.protocol,
        color: config.color,
        fancy: config.fancy,
    };

    utils::save_config_file(new_config);
}
fn update_sort(sort: defines::SORTING) {
    let config = utils::read_config();

    let new_config = Config {
        username: config.username,
        token: config.token,
        sort,
        protocol: config.protocol,
        color: config.color,
        fancy: config.fancy,
    };

    utils::save_config_file(new_config);
}
fn update_protocol(protocol: defines::PROTOCOL) {
    let config = utils::read_config();

    let new_config = Config {
        username: config.username,
        token: config.token,
        sort: config.sort,
        protocol,
        color: config.color,
        fancy: config.fancy,
    };

    utils::save_config_file(new_config);
}
fn update_color(color: defines::COLOR) {
    let config = utils::read_config();

    let new_config = Config {
        username: config.username,
        token: config.token,
        sort: config.sort,
        protocol: config.protocol,
        color,
        fancy: config.fancy,
    };

    utils::save_config_file(new_config);
}
fn update_fancy(fancy: bool) {
    let config = utils::read_config();

    let new_config = Config {
        username: config.username,
        token: config.token,
        sort: config.sort,
        protocol: config.protocol,
        color: config.color,
        fancy,
    };

    utils::save_config_file(new_config);
}
