use crate::utils::out;
use serde::{Deserialize, Serialize};

pub mod defines;
pub mod utils;
mod git;
mod github;
mod config;

pub use github::login;
pub use config::load_config;

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
        config::create_config();
    } else if !utils::validate_config_file() {
        out::print_error("Config file is invalid. Creating a new one...\n");
        config::create_config();
    }

    // Check for a GitHub token
    if !github::check_token() {
        out::print_error("Error: GitHub token invalid.\n");
        login().await;

        std::thread::sleep(std::time::Duration::from_secs(2));
    }
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
