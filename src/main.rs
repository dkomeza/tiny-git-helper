use std::env;

mod utils;
use utils::out;

mod config;

fn print_help() {
    use std::io::{stdout, Write};

    stdout().flush().unwrap();

    println!("");
    println!("tgh - A GitHub CLI written in Rust");
    println!("Usage:");
    println!("    commands:");
    println!("        commit: commit changes");
    println!("        clone: clone a repository into a new directory");
    println!("        init: create an empty GitHub repository");
    println!("        login: login to GitHub");
    println!("        settings: view and edit settings");
    println!("        version: show the version of tgh");
    println!("")
}

struct Args {
    mode: String,
    args: Vec<String>,
}

impl Args {
    fn new() -> Args {
        let args: Vec<String> = env::args().skip(1).collect();

        if args.len() == 0 {
            return Args {
                mode: String::from(""),
                args: vec![],
            };
        }

        let mode = args[0].clone();

        if args.len() == 1 {
            return Args { mode, args: vec![] };
        }

        let args = args[1..].to_vec();

        Args { mode, args }
    }
}

#[tokio::main]
async fn main() {
    let args = Args::new();

    let _ = config::load_config();

    config::check_prerequisites(args.mode.clone());

    if args.mode.len() == 0 {
        out::print_error("Error: No command provided.\n");
        print_help();
        std::process::exit(1);
    }

    match args.mode.as_str() {
        "login" => {
            let _ = config::login().await;
        }
        "help" => print_help(),
        _ => {
            out::print_error(format!("Unknown command: {} \n", args.mode).as_str());
            print_help();
        }
    };
}
