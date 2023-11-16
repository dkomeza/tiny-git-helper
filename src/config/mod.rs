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
    Config {}
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
