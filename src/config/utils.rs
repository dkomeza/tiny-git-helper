use crate::config::defines;

pub fn handle_config_folder() {
    use home::home_dir;
    use std::fs::create_dir_all;

    let home = home_dir().unwrap();
    let config_path = format!("{}/.config/tgh", home.display());

    create_dir_all(config_path).unwrap();
}

pub fn get_config_path() -> String {
    use home::home_dir;

    let home = home_dir().unwrap();
    let config_path = format!("{}/.config/tgh/config.json", home.display());

    return config_path;
}

pub fn config_exists() -> bool {
    use std::path::Path;

    let config_path = get_config_path();
    let config_path = Path::new(&config_path);

    return config_path.exists();
}

pub fn read_config_content() -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let config_path = get_config_path();
    let mut config_file = File::open(config_path).unwrap();
    let mut config_contents = String::new();

    config_file.read_to_string(&mut config_contents).unwrap();

    return config_contents;
}

pub fn read_config() -> crate::config::Config {
    let config_contents = read_config_content();
    let config: crate::config::Config = serde_json::from_str(&config_contents).unwrap();

    return config;
}

pub fn validate_config_file() -> bool {
    use crate::config::Config;

    let config_contents = read_config_content();

    if config_contents.len() == 0 {
        return false;
    }

    let config = serde_json::from_str(&config_contents);

    if config.is_err() {
        return false;
    }

    let config: Config = config.unwrap();

    if config.username.len() == 0 || config.token.len() == 0 {
        return false;
    }

    if config.sort < defines::SORTING::LastUpdated || config.sort > defines::SORTING::Alphabetical {
        return false;
    }

    if config.protocol < defines::PROTOCOL::SSH || config.protocol > defines::PROTOCOL::HTTPS {
        return false;
    }

    if config.color < defines::COLOR::NORMAL || config.color > defines::COLOR::GRAY {
        return false;
    }

    if config.fancy != true && config.fancy != false {
        return false;
    }

    return true;
}

pub fn save_config_file(config: crate::config::Config) {
    use std::{fs::File, io::prelude::*};

    let config_path = get_config_path();

    let mut config_file = File::create(config_path).unwrap();

    let config_contents = serde_json::to_string_pretty(&config).unwrap();

    config_file.write_all(config_contents.as_bytes()).unwrap();
}
