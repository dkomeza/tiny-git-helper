pub mod help;

pub fn menu(config: crate::config::Config) {
    use inquire::Select;
    use std::process;

    let choice;

    let menu = Select::new(
        "What do you want to do?",
        vec!["commit", "clone", "init", "login", "settings", "version"],
    )
    .prompt();

    match menu {
        Ok(option) => {
            choice = option;
        }
        Err(_) => {
            process::exit(0);
        }
    }

    match choice {
        "commit" => {
            println!("commit");
        }
        "clone" => {
            println!("clone");
        }
        "init" => {
            println!("init");
        }
        "login" => {
            println!("login");
        }
        "settings" => {
            println!("settings");
        }
        "version" => {
            println!("version");
        }
        _ => {
            println!("Invalid option");
        }
    }
}
