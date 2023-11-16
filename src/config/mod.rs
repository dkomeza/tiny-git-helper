use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    username: String,
    token: String,
    sort: i8,
    protocol: i8,
    color: i8,
    fancy: bool,
}

pub fn load_config() -> Config {
    if config_exists() {
        if validate_config_file() {
            return read_config();
        } else {
            return create_config();
        }
    }

    return create_config();
}

fn get_config_path() -> String {
    use home::home_dir;

    let home = home_dir().unwrap();
    let config_path = format!("{}/.config/tgh/config.json", home.display());

    return config_path;
}

fn config_exists() -> bool {
    use std::path::Path;

    let config_path = get_config_path();
    let config_path = Path::new(&config_path);

    return config_path.exists();
}

fn read_config_file() -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let config_path = get_config_path();
    let mut config_file = File::open(config_path).unwrap();
    let mut config_contents = String::new();

    config_file.read_to_string(&mut config_contents).unwrap();

    return config_contents;
}

fn validate_config_file() -> bool {
    let config_contents = read_config_file();

    if config_contents.len() == 0 {
        return false;
    }

    let config: Config = serde_json::from_str(&config_contents).unwrap();

    if config.username.len() == 0 || config.token.len() == 0 {
        return false;
    }

    if config.sort < 0 || config.sort > 1 {
        return false;
    }

    if config.protocol < 0 || config.protocol > 1 {
        return false;
    }

    if config.color < 0 || config.color > 8 {
        return false;
    }

    if config.fancy != true && config.fancy != false {
        return false;
    }

    return true;
}

fn read_config() -> Config {
    let config_contents = read_config_file();
    let config: Config = serde_json::from_str(&config_contents).unwrap();

    return config;
}
