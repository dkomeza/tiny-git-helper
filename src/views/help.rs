const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn print_help(error: bool, args: crate::Args) {
    use crate::clear_screen;
    use std::io::{stdout, Write};

    clear_screen();

    if error {
        crate::out::print_error(format!("Unknown command: {}", args.mode).as_str());
    }

    stdout().flush().unwrap();

    println!("");
    println!("tgh - A GitHub CLI written in Rust");
    println!("");
    println!("Usage:");
    println!("  commands:");
    println!("      commit: commit changes");
    println!("      clone: clone a repository into a new directory");
    println!("      init: create an empty GitHub repository");
    println!("      login: login to GitHub");
    println!("      settings: view and edit settings");
    println!("      version: show the version of tgh");
    println!("  short commands:");
    println!("      c: commit changes");
    println!("      ca: commit all changes");
    println!("")
}

pub fn print_version() {
    println!("tgh version: {}", VERSION);
}
