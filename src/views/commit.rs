pub fn commit_menu(config: crate::config::Config) {
    use crate::clear_screen;
    use inquire::Select;
    use std::process;

    clear_screen();

    let choice;

    let menu = Select::new("What do you want to do?", vec!["commit", "commit all"]).prompt();

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
        "commit all" => {
            println!("commit all");
        }
        _ => {
            println!("Invalid option");
        }
    }
}
