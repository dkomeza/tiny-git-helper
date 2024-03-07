use std::thread;

use crossterm::{
    cursor::{MoveLeft, MoveUp},
    terminal::{self, disable_raw_mode, enable_raw_mode},
};

use crate::utils::out::{format_bold, format_color, format_dim, format_underline};

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

    let size = crossterm::terminal::size().unwrap();

    let mut window_size = size.1 as usize - 3;

    if window_size > commits.len() {
        window_size = commits.len();
    }

    if window_size > 10 {
        window_size = 10;
    }

    let mut index = 0;
    let mut selected_index = 0;
    let max_index = commits.len() - 1 - window_size;

    for i in index..index + window_size {
        if i < commits.len() {
            let commit = &commits[i];
            let hash = format_dim(format!("({})", commit.hash).as_str());
            let message = commit.message.as_str();
            let date = format_color(commit.date.as_str(), crate::utils::out::Color::Green);
            let author = format_color(commit.author.as_str(), crate::utils::out::Color::Blue);

            if i == selected_index {
                print!("> ");
            } else {
                print!("  ");
            }

            println!("{} - {} ({}) ~ {}", hash, message, date, author);
        }
    }

    enable_raw_mode().unwrap();

    loop {
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::Key(event) => {
                if event.code == crossterm::event::KeyCode::Down {
                    if selected_index < commits.len() - 1 {
                        selected_index += 1;
                    }

                    if index < max_index && selected_index >= index + window_size {
                        index += 1;
                    }

                    render_commits(&commits, index, window_size, selected_index);
                } else if event.code == crossterm::event::KeyCode::Up {
                    if selected_index > 0 {
                        selected_index -= 1;
                    }

                    if index > 0 && selected_index < index {
                        index -= 1;
                    }

                    render_commits(&commits, index, window_size, selected_index);
                } else if event.code == crossterm::event::KeyCode::Char('q') {
                    break;
                } else if event.code == crossterm::event::KeyCode::Enter {
                    render_commit(&commits[selected_index], window_size);
                    return;
                }
            }
            _ => {}
        }
    }

    disable_raw_mode().unwrap();
}

fn render_commits(commits: &Vec<Commit>, index: usize, window_size: usize, selected_index: usize) {
    use crossterm::execute;
    use std::io::{stdout, Write};

    let mut stdout = stdout();

    let _ = execute!(stdout, MoveUp(window_size as u16));
    let _ = execute!(stdout, terminal::Clear(terminal::ClearType::FromCursorDown));

    for i in index..index + window_size {
        if i < commits.len() {
            let commit = &commits[i];
            let hash = format_dim(format!("({})", commit.hash).as_str());
            let message = commit.message.as_str();
            let date = format_color(commit.date.as_str(), crate::utils::out::Color::Green);
            let author = format_color(commit.author.as_str(), crate::utils::out::Color::Blue);

            execute!(stdout, MoveLeft(1000)).unwrap();

            if i == selected_index {
                execute!(stdout, terminal::Clear(terminal::ClearType::CurrentLine)).unwrap();
                print!("> ");
            } else {
                execute!(stdout, terminal::Clear(terminal::ClearType::CurrentLine)).unwrap();
                print!("  ");
            }
            writeln!(stdout, "{} - {} ({}) ~ {}", hash, message, date, author).unwrap();
        }
    }
    execute!(stdout, MoveLeft(1000)).unwrap();
    stdout.flush().unwrap();
}
fn render_commit(commit: &Commit, window_size: usize) {
    use crossterm::execute;
    use std::io::{stdout, Write};
    use std::process::Command;

    let mut stdout = stdout();

    let _ = execute!(stdout, MoveUp(window_size as u16));
    let _ = execute!(stdout, terminal::Clear(terminal::ClearType::FromCursorDown));

    disable_raw_mode().unwrap();

    let hash = commit.hash.as_str();

    let mut binding = Command::new("git");
    let command = binding
        .arg("show")
        .arg(hash)
        .arg("--pretty=format:%H-_-%an-_-%ae-_-%ad-_-%s-_-%b")
        .arg("--color")
        .arg("--compact-summary");

    let output = command.output().expect("Failed to execute git show");

    let out = String::from_utf8(output.stdout).unwrap();

    let binding = out.clone();
    let header = binding.lines().next().unwrap();
    let out = out.replace(header, "");

    let parts: Vec<&str> = header.split("-_-").collect();

    let hash = parts[0];
    let author = parts[1];
    let email = parts[2];
    let date = parts[3];
    let subject = parts[4];
    let body = parts[5];

    use crate::utils::out::Color;

    println!("");
    println!("Hash: ({})", format_dim(hash));
    println!(
        "Author: {} <{}>",
        format_color(author, Color::Blue),
        format_underline(format_color(email, Color::Magenta).as_str())
    );
    println!("Date: {}", format_color(date, Color::Green));
    println!(
        "Subject: {}",
        format_bold(format_color(subject, Color::Yellow).as_str())
    );

    if !body.is_empty() {
        println!("\nBody: {}", format_color(body, Color::Cyan));
    }

    print!("\nChanges:");
    println!("{}", out);
}
