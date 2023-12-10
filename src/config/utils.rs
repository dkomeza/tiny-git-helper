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
        emoji: ":bookmark:".into(),
    });
    labels.push(CommitLabel {
        label: "New feature".into(),
        emoji: ":sparkles:".into(),
    });
    labels.push(CommitLabel {
        label: "Bug fix".into(),
        emoji: ":bug:".into(),
    });
    labels.push(CommitLabel {
        label: "Metadata".into(),
        emoji: ":card_index:".into(),
    });
    labels.push(CommitLabel {
        label: "Documentation".into(),
        emoji: ":books:".into(),
    });
    labels.push(CommitLabel {
        label: "Documenting source code".into(),
        emoji: ":bulb:".into(),
    });
    labels.push(CommitLabel {
        label: "Performance".into(),
        emoji: ":racehorse:".into(),
    });
    labels.push(CommitLabel {
        label: "Cosmetic".into(),
        emoji: ":lipstick:".into(),
    });
    labels.push(CommitLabel {
        label: "Tests".into(),
        emoji: ":rotating_light:".into(),
    });
    labels.push(CommitLabel {
        label: "Adding a test".into(),
        emoji: ":white_check_mark:".into(),
    });
    labels.push(CommitLabel {
        label: "Make a test pass".into(),
        emoji: ":heavy_check_mark:".into(),
    });
    labels.push(CommitLabel {
        label: "General update".into(),
        emoji: ":zap:".into(),
    });
    labels.push(CommitLabel {
        label: "Improve format/structure".into(),
        emoji: ":art:".into(),
    });
    labels.push(CommitLabel {
        label: "Refactor code".into(),
        emoji: ":hammer:".into(),
    });
    labels.push(CommitLabel {
        label: "Removing code/files".into(),
        emoji: ":fire:".into(),
    });
    labels.push(CommitLabel {
        label: "Continuous Integration".into(),
        emoji: ":green_heart:".into(),
    });
    labels.push(CommitLabel {
        label: "Security".into(),
        emoji: ":lock:".into(),
    });
    labels.push(CommitLabel {
        label: "Upgrading dependencies".into(),
        emoji: ":arrow_up:".into(),
    });
    labels.push(CommitLabel {
        label: "Downgrading dependencies".into(),
        emoji: ":arrow_down:".into(),
    });
    labels.push(CommitLabel {
        label: "Lint".into(),
        emoji: ":shirt:".into(),
    });
    labels.push(CommitLabel {
        label: "Translation".into(),
        emoji: ":alien:".into(),
    });
    labels.push(CommitLabel {
        label: "Text".into(),
        emoji: ":pencil:".into(),
    });
    labels.push(CommitLabel {
        label: "Critical hotfix".into(),
        emoji: ":ambulance:".into(),
    });
    labels.push(CommitLabel {
        label: "Deploying stuff".into(),
        emoji: ":rocket:".into(),
    });
    labels.push(CommitLabel {
        label: "Fixing on MacOS".into(),
        emoji: ":apple:".into(),
    });
    labels.push(CommitLabel {
        label: "Fixing on Linux".into(),
        emoji: ":penguin:".into(),
    });
    labels.push(CommitLabel {
        label: "Fixing on Windows".into(),
        emoji: ":checkered_flag:".into(),
    });
    labels.push(CommitLabel {
        label: "Work in progress".into(),
        emoji: ":construction:".into(),
    });
    labels.push(CommitLabel {
        label: "Adding CI build system".into(),
        emoji: ":construction_worker:".into(),
    });
    labels.push(CommitLabel {
        label: "Analytics or tracking code".into(),
        emoji: ":chart_with_upwards_trend:".into(),
    });
    labels.push(CommitLabel {
        label: "Removing a dependency".into(),
        emoji: ":heavy_minus_sign:".into(),
    });
    labels.push(CommitLabel {
        label: "Adding a dependency".into(),
        emoji: ":heavy_plus_sign:".into(),
    });
    labels.push(CommitLabel {
        label: "Docker".into(),
        emoji: ":whale:".into(),
    });
    labels.push(CommitLabel {
        label: "Configuration files".into(),
        emoji: ":wrench:".into(),
    });
    labels.push(CommitLabel {
        label: "Package.json in JS".into(),
        emoji: ":package:".into(),
    });
    labels.push(CommitLabel {
        label: "Merging branches".into(),
        emoji: ":twisted_rightwards_arrows:".into(),
    });
    labels.push(CommitLabel {
        label: "Bad code/need improv.".into(),
        emoji: ":hankey:".into(),
    });
    labels.push(CommitLabel {
        label: "Reverting changes".into(),
        emoji: ":rewind:".into(),
    });
    labels.push(CommitLabel {
        label: "Breaking changes".into(),
        emoji: ":boom:".into(),
    });
    labels.push(CommitLabel {
        label: "Code review changes".into(),
        emoji: ":ok_hand:".into(),
    });
    labels.push(CommitLabel {
        label: "Accessibility".into(),
        emoji: ":wheelchair:".into(),
    });
    labels.push(CommitLabel {
        label: "Move/rename repository".into(),
        emoji: ":truck:".into(),
    });

    return labels;
}
