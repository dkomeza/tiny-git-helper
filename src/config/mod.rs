use crate::{utils::out, view};
use git::check_git_config;
use serde::{Deserialize, Serialize};

mod config;
pub mod defines;
mod git;
mod github;
pub mod utils;

pub use config::load_config;
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
pub async fn check_prerequisites() {
    match git::validate_git_install() {
        Ok(_) => {}
        Err(err) => {
            view::printer(&err.to_string());
            std::process::exit(1);
        }
    }

    // Check for git config
    match check_git_config() {
        Ok(_) => {}
        Err(err) => match err {
            git::GitConfigError::NameNotFound => {
                let msg = r#"
                    $b$cr `error`: Git user.name not found.

                    You can set it using the following command:
                    $i ` git config user.name "Your Name"`
                    
                    or globally:
                    $i ` git config --global user.name "Your Name"`
                    $i$s `this will not work if you set it locally`
                "#;
                view::printer(msg);
                std::process::exit(1);
            }
            git::GitConfigError::EmailNotFound => {
                let msg = r#"
                    $b$cr `error`: Git user.email not found.

                    You can set it using the following command:
                    $i ` git config user.email "`

                    or globally:
                    $i ` git config --global user.email "`
                    $i$s `this will not work if you set it locally`
                "#;
                view::printer(msg);
                std::process::exit(1);
            }
        },
    }

    // Check for a config file
    if !utils::config_exists() {
        view::printer("\n$b$cr `error`: Config file not found. Creating a new one...\n");
        config::create_config();
    } else if !utils::validate_config_file() {
        view::printer("\n$b$cr `error`: Config file is invalid. Creating a new one...\n");
        config::create_config();
    }

    // Check for a GitHub token
    if !github::check_token() {
        view::printer("\n$b$cr `error`: GitHub token not found. Logging in...\n");
        login().await;

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
