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

// Create a error message for when git is not installed depending on the OS
#[cfg(target_os = "windows")]
static GIT_NOT_INSTALLED: &str = r#"
$b$cr `error`: $b `git` is not installed.

You can install it using Chocolatey:
$i ` choco install git`

or using Winget:
$i ` winget install --id Git.Git -e --source winget`

Or you can download it from the official website:
$b ` `$u `https://git-scm.com/download/win`
"#;

#[cfg(target_os = "macos")]
static GIT_NOT_INSTALLED: &str = r#"
$b$cr `error`: $b `git` is not installed.

You can install it using Homebrew:
$i ` brew install git`

or using MacPorts:
$i ` sudo port install git`

Xcode also includes git. You can install it from the App Store.
"#;

#[cfg(target_os = "linux")]
static GIT_NOT_INSTALLED: &str = r#"
$b$cr `error`: $b `git` is not installed.

You can install it using your package manager.

For example, on Ubuntu:
$i ` sudo apt install git`

On Fedora:
$i ` sudo dnf install git`

On Arch Linux:
$i ` sudo pacman -S git`

You can also download it from the official website:
$b ` `$u `https://git-scm.com/download/linux`
"#;

/// Checks if the prerequisites for tgh are installed.
/// If not, it will print an error and exit.
pub async fn check_prerequisites() {
    // Check if git is installed
    if !git::check_git() {
        #[cfg(target_os = "windows")]
        view::printer(GIT_NOT_INSTALLED);

        #[cfg(target_os = "macos")]
        view::printer(GIT_NOT_INSTALLED);

        #[cfg(target_os = "linux")]
        view::printer(&utils::get_git_installation_instructions());
        std::process::exit(1);
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
