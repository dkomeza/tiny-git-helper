use crate::config::defines;

pub fn handle_config_folder() {
    use home::home_dir;
    use std::fs::create_dir_all;

    let home = home_dir().unwrap();
    let config_path = format!("{}/.config/tgh", home.display());

    create_dir_all(config_path).unwrap();
}

fn get_config_path() -> String {
    use home::home_dir;

    let home = home_dir().unwrap();
    let config_path = format!("{}/.config/tgh/config.json", home.display());

    config_path
}

fn get_metadata_path() -> String {
    use home::home_dir;

    let home = home_dir().unwrap();
    let metadata_path = format!("{}/.config/tgh/metadata.json", home.display());

    metadata_path
}

pub fn config_exists() -> bool {
    use std::path::Path;

    let config_path = get_config_path();
    let config_path = Path::new(&config_path);

    config_path.exists()
}

fn read_file_content(path: String) -> Result<String, std::io::Error> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut config_file = File::open(path)?;
    let mut config_contents = String::new();

    config_file.read_to_string(&mut config_contents)?;

    Ok(config_contents)
}

pub fn read_config() -> crate::config::Config {
    let config_contents = read_file_content(get_config_path());

    let conf = match config_contents {
        Err(_) => {
            panic!("Failed to read config file.");
        }
        Ok(contents) => contents,
    };

    let config: crate::config::Config = serde_json::from_str(&conf).unwrap();

    config
}

pub fn read_metadata() -> crate::config::Metadata {
    let metadata_path = get_metadata_path();

    let metadata_contents = read_file_content(metadata_path);

    let metadata = match metadata_contents {
        Err(_) => {
            let metadata = crate::config::Metadata::default();
            save_metadata_file(metadata.clone());
            metadata
        }
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
    };

    metadata
}

pub fn validate_config_file() -> bool {
    use crate::config::Config;

    let config_contents = match read_file_content(get_config_path()) {
        Ok(contents) => contents,
        Err(_) => {
            return false;
        }
    };

    if config_contents.len() == 0 {
        return false;
    }

    let config = serde_json::from_str(&config_contents);

    if config.is_err() {
        return false;
    }

    let config: Config = config.unwrap();

    if config.username.len() == 0 {
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

    true
}

pub fn save_config_file(config: crate::config::Config) {
    use std::{fs::File, io::prelude::*};

    let config_path = get_config_path();

    let mut config_file = File::create(config_path).unwrap();

    let config_contents = serde_json::to_string_pretty(&config).unwrap();

    config_file.write_all(config_contents.as_bytes()).unwrap();
}

#[derive(Clone, Debug)]
pub struct CommitLabel {
    pub label: String,
    pub emoji: String,
}

impl std::fmt::Display for CommitLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.emoji, self.label)
    }
}

pub fn get_labels() -> Vec<CommitLabel> {
    let mut labels = Vec::new();

    labels.push(CommitLabel {
        label: "Initial commit".into(),
        emoji: "🎉".into(),
    });
    labels.push(CommitLabel {
        label: "Version tag".into(),
        emoji: "🔖".into(),
    });
    labels.push(CommitLabel {
        label: "New feature".into(),
        emoji: "✨".into(),
    });
    labels.push(CommitLabel {
        label: "Bug fix".into(),
        emoji: "🐛".into(),
    });
    labels.push(CommitLabel {
        label: "Metadata".into(),
        emoji: "📇".into(),
    });
    labels.push(CommitLabel {
        label: "Documentation".into(),
        emoji: "📚".into(),
    });
    labels.push(CommitLabel {
        label: "Documenting source code".into(),
        emoji: "💡".into(),
    });
    labels.push(CommitLabel {
        label: "Performance".into(),
        emoji: "🏇".into(),
    });
    labels.push(CommitLabel {
        label: "Cosmetic".into(),
        emoji: "💄".into(),
    });
    labels.push(CommitLabel {
        label: "Tests".into(),
        emoji: "🚨".into(),
    });
    labels.push(CommitLabel {
        label: "Adding a test".into(),
        emoji: "📋".into(),
    });
    labels.push(CommitLabel {
        label: "Make a test pass".into(),
        emoji: "✅".into(),
    });
    labels.push(CommitLabel {
        label: "General update".into(),
        emoji: "📂".into(),
    });
    labels.push(CommitLabel {
        label: "Improve format/structure".into(),
        emoji: "🎨".into(),
    });
    labels.push(CommitLabel {
        label: "Refactor code".into(),
        emoji: "🔨".into(),
    });
    labels.push(CommitLabel {
        label: "Removing code/files".into(),
        emoji: "🔥".into(),
    });
    labels.push(CommitLabel {
        label: "Continuous Integration".into(),
        emoji: "💚".into(),
    });
    labels.push(CommitLabel {
        label: "Security".into(),
        emoji: "🔒".into(),
    });
    labels.push(CommitLabel {
        label: " Upgrading dependencies".into(),
        emoji: "⬆️".into(),
    });
    labels.push(CommitLabel {
        label: " Downgrading dependencies".into(),
        emoji: "⬇️".into(),
    });
    labels.push(CommitLabel {
        label: "Lint".into(),
        emoji: "👕".into(),
    });
    labels.push(CommitLabel {
        label: "Translation".into(),
        emoji: "👽".into(),
    });
    labels.push(CommitLabel {
        label: "Text".into(),
        emoji: "📝".into(),
    });
    labels.push(CommitLabel {
        label: "Critical hotfix".into(),
        emoji: "🚑".into(),
    });
    labels.push(CommitLabel {
        label: "Deploying stuff".into(),
        emoji: "🚀".into(),
    });
    labels.push(CommitLabel {
        label: "Fixing on MacOS".into(),
        emoji: "🍎".into(),
    });
    labels.push(CommitLabel {
        label: "Fixing on Linux".into(),
        emoji: "🐧".into(),
    });
    labels.push(CommitLabel {
        label: "Fixing on Windows".into(),
        emoji: "🏁".into(),
    });
    labels.push(CommitLabel {
        label: "Work in progress".into(),
        emoji: "🚧".into(),
    });
    labels.push(CommitLabel {
        label: "Adding CI build system".into(),
        emoji: "👷".into(),
    });
    labels.push(CommitLabel {
        label: "Analytics or tracking code".into(),
        emoji: "📈".into(),
    });
    labels.push(CommitLabel {
        label: "Removing a dependency".into(),
        emoji: "➖".into(),
    });
    labels.push(CommitLabel {
        label: "Adding a dependency".into(),
        emoji: "➕".into(),
    });
    labels.push(CommitLabel {
        label: "Docker".into(),
        emoji: "🐳".into(),
    });
    labels.push(CommitLabel {
        label: "Configuration files".into(),
        emoji: "🔧".into(),
    });
    labels.push(CommitLabel {
        label: "Package.json in JS".into(),
        emoji: "📦".into(),
    });
    labels.push(CommitLabel {
        label: "Merging branches".into(),
        emoji: "🔀".into(),
    });
    labels.push(CommitLabel {
        label: "Bad code/need improv.".into(),
        emoji: "📑".into(),
    });
    labels.push(CommitLabel {
        label: " Reverting changes".into(),
        emoji: "⏮️".into(),
    });
    labels.push(CommitLabel {
        label: "Breaking changes".into(),
        emoji: "💥".into(),
    });
    labels.push(CommitLabel {
        label: "Code review changes".into(),
        emoji: "👍".into(),
    });
    labels.push(CommitLabel {
        label: "Accessibility".into(),
        emoji: "🦽".into(),
    });
    labels.push(CommitLabel {
        label: "Move/rename repository".into(),
        emoji: "🚚".into(),
    });

    labels
}

pub fn should_check_for_updates() -> bool {
    let metadata = read_metadata();

    let now = chrono::Utc::now();
    let last_checked = chrono::DateTime::parse_from_rfc3339(&metadata.last_checked)
        .unwrap()
        .to_utc();

    let time_diff = now.signed_duration_since(last_checked).num_days();

    if time_diff >= 1 {
        return true;
    }

    false
}

pub fn save_metadata_file(metadata: crate::config::Metadata) {
    use std::{fs::File, io::prelude::*};

    let metadata_path = get_metadata_path();

    let mut metadata_file = match (File::create(metadata_path)) {
        Ok(file) => file,
        Err(err) => {
            panic!("Failed to create metadata file: {}", err.to_string());
        }
    };

    let metadata_contents = serde_json::to_string_pretty(&metadata).unwrap();

    metadata_file
        .write_all(metadata_contents.as_bytes())
        .unwrap();
}
