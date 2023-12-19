use super::utils;
use super::Config;
use crate::out;

pub fn check_token() -> bool {
    if !utils::config_exists() || !utils::validate_config_file() {
        return false;
    }

    let config = utils::read_config();

    if config.token.len() == 0 {
        return false;
    }

    return true;
}

/// Authenticate user with GitHub.
/// @TODO: Split this function into smaller functions.
pub async fn authenticate() -> Result<String, reqwest::Error> {
    use arboard::Clipboard;

    let client_id = "Iv1.d8c9cc38202b9305";
    let client = reqwest::Client::new();

    let mut text = client
        .post(format!(
            "https://github.com/login/device/code?client_id={}",
            client_id
        ))
        .send()
        .await?
        .text()
        .await?;

    text = text.replace('"', "").to_string();
    let text_split: Vec<String> = text
        .split("&")
        .map(|s| s.split("=").map(|s| s.to_string()).collect::<Vec<_>>()[1].to_string())
        .collect();

    let device_code = text_split[0].to_string();
    let expires_in = text_split[1].parse::<u64>().unwrap();
    let interval = text_split[2].parse::<u64>().unwrap();
    let user_code = text_split[3].to_string();
    let login_url = text_split[4].replace("%3A", ":").replace("%2F", "/");
    let grant_type = "urn:ietf:params:oauth:grant-type:device_code";

    println!(
        "Please visit this URL to authenticate: \x1B[4m{}\x1B[m",
        login_url
    );

    let clipboard = Clipboard::new();
    match clipboard {
        Ok(mut clipboard) => {
            clipboard.set_text(user_code.clone()).unwrap();
            println!(
                "Your user code has been copied to your clipboard. ({})",
                user_code
            )
        }
        Err(_) => {
            println!(
                "Error copying to clipboard, copy the code manually: {}",
                user_code
            );
        }
    }

    let params = [
        ("client_id", client_id),
        ("device_code", &device_code),
        ("grant_type", grant_type),
    ];

    let start_time = std::time::Instant::now();
    let token;

    loop {
        let res = client
            .post("https://github.com/login/oauth/access_token")
            .header(reqwest::header::ACCEPT, "application/json")
            .form(&params)
            .send()
            .await?;

        let mut res = res.text().await?;

        if res.contains("access_token") {
            res = res
                .replace("{", "")
                .replace("}", "")
                .replace('"', "")
                .replace("/", "");
            let res_split: Vec<String> = res
                .split(",")
                .map(|s| s.split(":").map(|s| s.to_string()).collect::<Vec<_>>()[1].to_string())
                .collect();

            token = res_split[0].to_string();

            break;
        }

        // Check if the authentication timed out
        if std::time::Instant::now()
            .duration_since(start_time)
            .as_secs()
            > (expires_in)
        {
            println!("Authentication timed out.");
            std::process::exit(1);
        }

        // Wait for the interval
        std::thread::sleep(std::time::Duration::from_secs(interval));
    }

    return Ok(token);
}

pub async fn login(args: &crate::Args) {
    if args.help {
        out::print_bold("tgh login - Login to GitHub");
        println!("");
        println!("Logs in to GitHub and saves the token to the config file.");
        println!("");
        out::print_bold("Usage:");
        println!("      tgh login [options]");
        println!("");
        out::print_bold("Options:");
        println!("      -h | --help: show this help message");
        println!("");
        return;
    }

    let token = authenticate().await;

    match token {
        Ok(token) => {
            update_token(token.clone());

            out::print_success("Successfully authenticated.\n");
        }
        Err(err) => {
            println!("{:?}", err);
            out::print_error("Error: Failed to authenticate.\n");
            std::process::exit(1);
        }
    }
}

fn update_token(token: String) {
    let config = utils::read_config();

    let new_config = Config {
        username: config.username,
        token,
        sort: config.sort,
        protocol: config.protocol,
        color: config.color,
        fancy: config.fancy,
    };

    utils::save_config_file(new_config);
}
