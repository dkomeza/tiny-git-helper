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

    let branch = match options.branch {
        Some(branch) => branch,
        None => "".into(),
    };

    let file = match options.file {
        Some(file) => file,
        None => "".into(),
    };

    let output = Command::new("git")
        .arg("log")
        .arg("--oneline")
        .arg("--decorate")
        .arg("--all")
        .arg("--color")
        .arg("--pretty=format:%h_%s_%cr_%an")
        .arg("--full-history")
        .arg(format!("-{}", limit))
        .arg(format!("--author={}", author))
        .output()
        .expect("Failed to execute git log");

    let out = String::from_utf8(output.stdout).unwrap();

    let commits: Vec<Commit> = out
        .lines()
        .map(|line| {
            let mut parts = line.split('_');
            Commit {
                hash: parts.next().unwrap().into(),
                message: parts.next().unwrap().into(),
                date: parts.next().unwrap().into(),
                author: parts.next().unwrap().into(),
            }
        })
        .collect();

    for commit in commits {
        let hash = format_dim(format!("({})", commit.hash).as_str());
        let message = commit.message;
        let date = format_color(commit.date.as_str(), crate::utils::out::Color::Green);
        let author = format_color(commit.author.as_str(), crate::utils::out::Color::Blue);

        println!("{} - {} ({}) ~ {}", hash, message, date, author);
    }
}
