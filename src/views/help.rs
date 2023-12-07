const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn print_help(error: bool, args: crate::Args) {
    use crate::{
        clear_screen,
        out::{print_bold, print_dim, print_error},
    };

    clear_screen();

    if error {
        print_error(format!("Unknown command: {}", args.mode).as_str());
    }

    println!("");
    print_bold("tgh - A GitHub CLI written in Rust");
    println!("");
    print_bold("Usage:");
    print_bold("  commands:");
    print_dim("      commit: commit changes (WIP)");
    print_dim("      clone: clone a repository into a new directory (WIP)");
    print_dim("      init: create an empty GitHub repository (WIP)");
    println!("      login: login to GitHub");
    print_dim("      settings: view and edit settings (WIP)");
    println!("      version: show the version of tgh");
    println!("      help: show this help message");
    print_bold("  short commands:");
    print_dim("      c: commit changes");
    print_dim("      ca: commit all changes");
    println!("")
}

pub fn print_version() {
    println!("tgh version: {}", VERSION);
}
