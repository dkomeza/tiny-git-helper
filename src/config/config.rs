use super::{defines, utils, Config};
use crate::out;

/// Loads the config file.
/// If the config file doesn't exist or is invalid, it will create a new one.
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
    create_config()
}

pub fn create_config() -> Config {
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

    config
}

fn ask_username() -> String {
    use inquire::{required, Text};

    let username = Text::new("Enter your GitHub username:")
        .with_validator(required!("Username is required."))
        .prompt();

    username.unwrap()
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
    use super::defines::COLOR::*;
    use inquire::Select;

    let option = Select::new(
        "Select a color for the output:",
        vec![NORMAL, RED, GREEN, YELLOW, BLUE, MAGENTA, CYAN, WHITE, GRAY],
    )
    .with_page_size(5)
    .prompt();

    match option {
        Ok(option) => option,
        Err(_) => {
            out::print_error("Invalid input.\n");
            ask_color()
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
            ask_fancy()
        }
    }
}
