mod utils;

use std::io::Write;
use utils::out;

#[tokio::main]
async fn main() {
    let mut supported_os = false;

    #[cfg(target_os = "windows")]
    {
        supported_os = true;
    }

    if !supported_os {
        out::print_error("This OS is not supported");
        // std::process::exit(1);
    }

    println!("Installing...");

    let url = "https://github.com/dkomeza/tiny-git-helper/releases/latest/download/";

    let file_name = format!("tgh-windows-x86_64-v{}.exe", env!("CARGO_PKG_VERSION"));
    let download_url = format!("{}{}", url, file_name);

    let response = reqwest::get(download_url).await.unwrap();

    let mut file = std::fs::File::create(file_name).unwrap();
    let content = response.bytes().await.unwrap();
    file.write_all(&content).unwrap();
}
