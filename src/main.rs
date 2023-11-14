use std::env;

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
            _ => {
                println!("Unknown command: {}", mode);
            }
        }
    }
}
