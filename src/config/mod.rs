use crate::utils::out;
use serde::{Deserialize, Serialize};

pub mod defines;
pub mod utils;
mod git;
mod github;

pub use github::login;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub username: String,
    pub token: String,
    pub sort: defines::SORTING,
    pub protocol: defines::PROTOCOL,
    pub color: defines::COLOR,
    pub fancy: bool,
}

/// Checks if the prerequisites for tgh are installed.
/// If not, it will print an error and exit.
///
/// ### Arguments
/// * `args` - A vector of the command line arguments.
pub async fn check_prerequisites() {
    // Check if git is installed
    if !git::check_git() {
        out::print_error("Error: Git is not installed.\n");
        println!("Please install using the link below:");
        println!("\x1B[mhttps://git-scm.com/downloads\x1B[m\n");
        std::process::exit(1);
    }

    // Check for git config
    git::check_git_config();

    // Check for a config file
    if !utils::config_exists() {
        out::print_error("Config file not found. Creating one...\n");
        create_config();
    } else if !utils::validate_config_file() {
        out::print_error("Config file is invalid. Creating a new one...\n");
        create_config();
    }

    // Check for a GitHub token
    if !github::check_token() {
        out::print_error("Error: GitHub token invalid.\n");
        login().await;

        std::thread::sleep(std::time::Duration::from_secs(2));
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

pub fn get_inquire_color() -> inquire::ui::Color {
    let config = load_config();
    use defines::COLOR;

    return match config.color {
        COLOR::RED => inquire::ui::Color::LightRed,
        COLOR::GREEN => inquire::ui::Color::LightGreen,
        COLOR::YELLOW => inquire::ui::Color::LightYellow,
        COLOR::BLUE => inquire::ui::Color::LightBlue,
        COLOR::MAGENTA => inquire::ui::Color::LightMagenta,
        COLOR::CYAN => inquire::ui::Color::LightCyan,
        COLOR::WHITE => inquire::ui::Color::White,
        COLOR::GRAY => inquire::ui::Color::Grey,
        _ => inquire::ui::Color::White,
    };
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
