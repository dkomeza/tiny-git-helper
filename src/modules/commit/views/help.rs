use crate::{clear_screen, out};

pub fn commit_help() {
    use out::print_bold;

    clear_screen();

    print_bold("tgh commit - Commit changes to a GitHub repository");
    println!("");
    println!("Opens the commit menu");
    println!("");
    print_bold("Usage:");
    println!("      tgh commit [options]");
    println!("");
    print_bold("Options:");
    println!("      -h | --help: show this help message");
    println!("      --no-push: don't push changes to remote");
    println!("      --skip-fancy: don't use fancy commit messages");
    println!("      --force-fancy: force fancy commit messages");
    println!("");
    print_bold("Short commands:");
    println!("      tgh ca: commit all changes");
    println!("      tgh cf: commit specific files");
    println!("");
}

pub fn commit_all_help() {
    use out::print_bold;

    clear_screen();

    print_bold("tgh commit all - Commit all changes to a GitHub repository");
    println!("");
    println!("Commits all changes to a GitHub repository and pushes them to the remote (fails if there is no remote (WIP))");
    println!("");
    print_bold("Usage:");
    println!("      tgh commit all [options]");
    println!("");
    print_bold("Options:");
    println!("      -h | --help: show this help message");
    println!("      --no-push: don't push changes to remote");
    println!("      --skip-fancy: don't use fancy commit messages");
    println!("      --force-fancy: force fancy commit messages");
    println!("");
}

pub fn commit_specific_help() {}
