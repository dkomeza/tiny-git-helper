use std::env;

mod utils;
use utils::out;

mod config;

fn print_help() {
    println!("Usage:");
    println!("    commands:");
    println!("        commit: commit changes");
    println!("        clone: clone a repository into a new directory");
    println!("        init: create an empty GitHub repository");
    println!("        settings: view and edit settings");
    println!("")
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    config::check_prerequisites(args.clone());

    let _ = config::load_config();

    if args.len() == 0 {
        out::print_error("Error: No command provided.\n");
        print_help();
        std::process::exit(1);
    }

    let mode = args[0].clone();

    match mode.as_str() {
        "login" => {
            let _ = config::login().await;
        }
        "help" => print_help(),
        _ => {
            out::print_error(format!("Unknown command: {} \n", mode).as_str());
            print_help();
        }
    };
}
