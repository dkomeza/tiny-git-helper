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

fn main() {
    // let config = config::load_config();
    config::check_prerequisites();

    let args: Vec<String> = env::args().skip(1).collect();

    // if args.len() == 0 {
    // } else {
    //     let mode = args[0].clone();

    //     match mode.as_str() {
    //         "commit" => println!("Committing..."),
    //         "clone" => println!("Cloning..."),
    //         "init" => println!("Initializing..."),
    //         "settings" => println!("Settings..."),
    //         "help" => print_help(),
    //         _ => {
    //             out::print_error(format!("Unknown command: {} \n", mode).as_str());
    //             print_help();
    //         }
    //     }
    // }
}
