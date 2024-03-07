use crate::utils::out::{format_color, format_dim};

use super::CommitHistoryOptions;

#[derive(Debug)]
struct Commit {
    hash: String,
    message: String,
    date: String,
    author: String,
}

pub fn commit_history(options: CommitHistoryOptions) {
    use std::process::Command;

    let limit = match options.limit {
        Some(limit) => limit.to_string(),
        None => "10".into(),
    };

    let author = match options.author {
        Some(author) => author,
        None => "".into(),
    };

    let mut branch = match options.branch {
        Some(branch) => branch,
        None => "".into(),
    };

    let file = match options.file {
        Some(file) => file,
        None => "".into(),
    };

    let all = options.all;

    if branch.is_empty() && !all {
        branch = super::functions::get_current_branch();
    }

    print!("\nShowing commits");
    if !file.is_empty() {
        print!(
            " for {}",
            format_color(file.as_str(), crate::utils::out::Color::Green)
        );
    }
    if !author.is_empty() {
        print!(
            " made by {}",
            format_color(author.as_str(), crate::utils::out::Color::Blue)
        );
    }
    println!(
        " on {}",
        format_color(branch.as_str(), crate::utils::out::Color::Yellow)
    );

    let mut binding = Command::new("git");
    let command = binding
        .arg("log")
        .arg("--oneline")
        .arg("--decorate")
        .arg("--color")
        .arg("--pretty=format:%h-_-%s-_-%cr-_-%an")
        .arg("--full-history")
        .arg(format!("-{}", limit))
        .arg(format!("--author={}", author))
        .arg(format!("{}", branch));

    if !file.is_empty() {
        command.arg(format!("{}", file));
    }

    let output = command.output().expect("Failed to execute git log");

    let out = String::from_utf8(output.stdout).unwrap();

    let commits: Vec<Commit> = out
        .lines()
        .map(|line| {
            let mut parts = line.split("-_-");
            Commit {
                hash: parts.next().unwrap().into(),
                message: parts.next().unwrap().into(),
                date: parts.next().unwrap().into(),
                author: parts.next().unwrap().into(),
            }
        })
        .collect();

    if commits.len() == 0 {
        crate::out::print_error("\nNo commits found\n");
        return;
    }

    for commit in commits {
        let hash = format_dim(format!("({})", commit.hash).as_str());
        let message = commit.message;
        let date = format_color(commit.date.as_str(), crate::utils::out::Color::Green);
        let author = format_color(commit.author.as_str(), crate::utils::out::Color::Blue);

        println!("{} - {} ({}) ~ {}", hash, message, date, author);
    }
}
