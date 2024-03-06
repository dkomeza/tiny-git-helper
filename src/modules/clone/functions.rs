use super::views::Repo;

pub fn clone_repo(repo: Repo) {
    use crate::config::{defines::PROTOCOL, load_config};
    use spinners::{Spinner, Spinners};
    use std::process::Command;

    let config = load_config();

    let url = if config.protocol == PROTOCOL::SSH {
        repo.ssh_url
    } else {
        repo.clone_url
    };

    let mut spinner = Spinner::new(Spinners::Dots, "Cloning...".into());

    let output = Command::new("git").arg("clone").arg(url).output().unwrap();

    if !output.status.success() {
        crate::out::print_error("Failed to clone repository");

        let out = String::from_utf8(output.stderr).unwrap();
        crate::out::print_error(&out);
        std::process::exit(1);
    }

    spinner.stop_with_symbol("✔");

    crate::out::print_success("Repository cloned successfully");
}
