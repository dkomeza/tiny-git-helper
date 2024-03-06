pub async fn clone_menu(options: super::CloneOptions) {
    match options.public {
        true => {
            clone_public_repo();
        }
        false => {
            clone_private_repo().await;
        }
    }
}

#[derive(serde::Deserialize)]
pub struct Repo {
    pub name: String,
    pub full_name: String,
    pub ssh_url: String,
    pub clone_url: String,
}

impl std::fmt::Display for Repo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.full_name)
    }
}

async fn clone_private_repo() {
    use inquire::Select;
    use spinners::{Spinner, Spinners};

    let mut spinner = Spinner::new(Spinners::Dots9, "Getting repositories".into());

    // Get user repositories
    let repos = get_user_repos().await;

    spinner.stop_with_symbol("✔");

    let prompt = Select::new("Select repository", repos).prompt();

    match prompt {
        Ok(repo) => {
            super::functions::clone_repo(repo);
        }
        Err(_) => {}
    }
}

fn clone_public_repo() {}

async fn get_user_repos() -> Vec<Repo> {
    let config = crate::config::load_config();
    let url = "https://api.github.com/user/repos";
    let token = config.token;
    let user_agent = "tgh";
    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();

    headers.append(
        "Authorization",
        format!("Bearer {}", token).parse().unwrap(),
    );
    headers.append("Accept", "application/vnd.github+json".parse().unwrap());
    headers.append("X-GitHub-Api-Version", "2022-11-28".parse().unwrap());
    headers.append("User-Agent", user_agent.parse().unwrap());

    let response: Vec<Repo> = client
        .get(url)
        .headers(headers)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    return response;
}
