use std::env;

fn print_help() {
    println!("Usage:");
    println!("    commands:");
    println!("        commit: commit changes");
    println!("        clone: clone a repository into a new directory");
    println!("        init: create an empty GitHub repository");
    println!("        settings: view and edit settings");
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 0 {
    } else {
        let mode = args[0].clone();

        match mode.as_str() {
            "commit" => println!("Committing..."),
            "clone" => println!("Cloning..."),
            "init" => println!("Initializing..."),
            "settings" => println!("Settings..."),
            "help" => print_help(),
            _ => {
                println!("Unknown command: {}", mode);
                print_help();
            }
        }
    }
}
