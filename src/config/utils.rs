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

    return true;
}

pub fn save_config_file(config: crate::config::Config) {
    use std::{fs::File, io::prelude::*};

    let config_path = get_config_path();

    let mut config_file = File::create(config_path).unwrap();

    let config_contents = serde_json::to_string_pretty(&config).unwrap();

    config_file.write_all(config_contents.as_bytes()).unwrap();
}

pub fn validate_email(
    email: &str,
) -> Result<inquire::validator::Validation, inquire::CustomUserError> {
    use regex::Regex;

    let re = Regex::new(r"^[\w-\.]+@([\w-]+\.)+[\w-]{2,4}$").unwrap();

    if email.len() == 0 {
        return Ok(inquire::validator::Validation::Invalid(
            "Email cannot be empty".into(),
        ));
    }

    if !re.is_match(email) {
        return Ok(inquire::validator::Validation::Invalid(
            "Invalid email".into(),
        ));
    }

    return Ok(inquire::validator::Validation::Valid);
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

    return labels;
}
