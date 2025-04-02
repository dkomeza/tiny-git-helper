const MIN_GIT_VERSION: &str = "2.20.0";

// Create an error message for when git is not installed depending on the OS
#[cfg(target_os = "windows")]
static GIT_INSTALL_INSTRUCTIONS: &str = r#"
You can install it using Chocolatey:
$i ` choco install git`

or using Winget:
$i ` winget install --id Git.Git -e --source winget`

Or you can download it from the official website:
$b ` `$u `https://git-scm.com/download/win`
"#;

#[cfg(target_os = "macos")]
static GIT_INSTALL_INSTRUCTIONS: &str = r#"
You can install it using Homebrew:
$i ` brew install git`

or using MacPorts:
$i ` sudo port install git`

Xcode also includes git. You can install it from the App Store.
"#;

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
static GIT_INSTALL_INSTRUCTIONS: &str = r#"
You can install it using your package manager.

Or you can download it from the official website:
$b ` `$u `https://git-scm.com/`
"#;

pub enum GitError {
    NotInstalled,
    VersionNotSupported { current: String, min: String },
}

impl GitError {
    pub fn to_string(&self) -> String {
        match self {
            GitError::NotInstalled => {
                #[cfg(target_os = "linux")] // For Linux, use the dynamic message (based on distro)
                let message = get_git_installation_instructions();

                #[cfg(not(target_os = "linux"))] // For other OSes, use the static message
                let message = GIT_INSTALL_INSTRUCTIONS;

                return format!(
                    r#"
                    $b$cr `error`: $b `git` is not installed.
                    
                    {}"#,
                    message,
                );
            }

            GitError::VersionNotSupported { current, min } => {
                #[cfg(target_os = "linux")]
                let install_cmd = get_git_installation_instructions();

                #[cfg(not(target_os = "linux"))]
                let install_cmd = GIT_INSTALL_INSTRUCTIONS;

                let msg = format!(
                    r#"
                    $b$cr `error`: $b `git` version not supported.
                    You need at least version {}, you are currently using version {}

                    {}
                "#,
                    min, current, install_cmd
                );

                return msg;
            }
        }
    }
}

pub fn validate_git_install() -> Result<(), GitError> {
    let mut command = std::process::Command::new("git");
    command.arg("--version");

    let output = command.output().unwrap();

    if !output.status.success() {
        return Err(GitError::NotInstalled);
    }

    let binding = String::from_utf8(output.stdout).unwrap();
    let s = binding.trim();
    if s.len() == 0 {
        return Err(GitError::NotInstalled);
    }

    let version = s.split(" ").last().unwrap();
    let version_u32 = version
        .split(".")
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let min_version = MIN_GIT_VERSION
        .split(".")
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    if version_u32.len() != 3 || min_version.len() != 3 {
        return Err(GitError::VersionNotSupported {
            current: version.to_string(),
            min: MIN_GIT_VERSION.to_string(),
        });
    }

    for i in 0..3 {
        if version_u32[i] < min_version[i] {
            return Err(GitError::VersionNotSupported {
                current: version.to_string(),
                min: MIN_GIT_VERSION.to_string(),
            });
        } else if version_u32[i] > min_version[i] {
            return Ok(());
        }
    }

    Ok(())
}

pub enum GitConfigError {
    NameNotFound,
    EmailNotFound,
}

/// Checks if the user has a git config. (user.name, user.email)
pub fn check_git_config() -> Result<(), GitConfigError> {
    let mut command = std::process::Command::new("git");
    command.args(["config", "user.name"]);

    let output = command.output().unwrap();
    let binding = String::from_utf8(output.stdout).unwrap();
    let s = binding.trim();

    if !output.status.success() || s.len() == 0 {
        return Err(GitConfigError::NameNotFound);
    }

    let mut command = std::process::Command::new("git");
    command.args(["config", "user.email"]);

    let output = command.output().unwrap();
    let binding = String::from_utf8(output.stdout).unwrap();
    let s = binding.trim();

    if !output.status.success() || s.len() == 0 {
        return Err(GitConfigError::EmailNotFound);
    }

    Ok(())
}

#[cfg(target_os = "linux")]
fn get_git_installation_instructions() -> String {
    let install_cmd;

    // Get the distribution
    let binding = match std::fs::read_to_string("/etc/os-release") {
        Ok(binding) => binding,
        Err(_) => {
            return r#"
            $b$cr `error`: $b `git` is not installed.
            
            You can download it from the official website:
            $b ` `$u `https://git-scm.com/download/linux`
            "#
            .into();
        }
    };
    let distro = binding
        .lines()
        .find(|line| line.starts_with("ID="))
        .unwrap()
        .split('=')
        .last()
        .unwrap();

    match distro {
        "ubuntu" | "debian" => {
            install_cmd = "sudo apt install git";
        }
        "fedora" | "centos" | "rhel" => {
            install_cmd = "sudo dnf install git";
        }
        "arch" | "manjaro" => {
            install_cmd = "sudo pacman -S git";
        }
        "alpine" => {
            install_cmd = "apk add git";
        }
        _ => {
            return r#"
            $b$cr `error`: $b `git` is not installed.
            
            You can download it from the official website:
            $b ` `$u `https://git-scm.com/download/linux`
            "#
            .into();
        }
    }

    let instructions = format!(
        r#"
        $b$cr `error`: $b `git` is not installed.

        You can install it using your package manager:
        $i ` {}`

        Or you can download it from the official website:
        $b ` `$u `https://git-scm.com/download/linux`
        "#,
        install_cmd
    );

    instructions
}
