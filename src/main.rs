use clap::{Parser, Subcommand};

mod utils;
use utils::out;
use utils::out::clear_screen;
use view::input;

mod config;
mod functions;
mod modules;
mod view;

#[derive(Parser)]
#[command(name = "tgh", author, version, about)]
struct Cli {
    #[clap(subcommand)]
    subcmd: Option<SubCommand>,
}

#[derive(Subcommand)]
enum SubCommand {
    #[clap(name = "commit", about = "Open the commit menu")]
    #[clap(visible_alias = "c")]
    Commit(modules::commit::CommitOptions),
    #[clap(name = "ca", about = "Commit all files")]
    CommitAll(modules::commit::CommitOptions),
    #[clap(name = "cf", about = "Commit specific files")]
    CommitFiles(modules::commit::CommitOptions),

    #[clap(name = "clone", about = "Clone a repository")]
    Clone(modules::clone::CloneOptions),

    #[clap(name = "history", about = "Show commit history")]
    #[clap(visible_alias = "log")]
    History(modules::history::CommitHistoryOptions),

    #[clap(name = "login", about = "Login to GitHub")]
    Login,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let password = match input::text("$cg `>` $cw `Enter your GitHub password: `") {
        Ok(password) => password,
        Err(err) => match err {
            input::ReturnType::Cancel => {
                println!("Cancelled");
                return;
            }
            input::ReturnType::Exit => {
                println!("Exiting");
                return;
            }
        },
    };
    println!("Password: {}", password);

    return;
    config::check_prerequisites().await;

    let subcmd = match args.subcmd {
        Some(subcmd) => subcmd,
        None => {
            view::no_subcommand_error();
            return;
        }
    };

    match subcmd {
        SubCommand::Commit(options) => {
            return modules::commit::commit_menu(options);
        }
        SubCommand::CommitAll(options) => {
            return modules::commit::commit_all_files(options);
        }
        SubCommand::CommitFiles(options) => {
            return modules::commit::commit_specific_files(options);
        }
        SubCommand::Clone(options) => {
            return modules::clone::clone_menu(options).await;
        }
        SubCommand::History(options) => {
            return modules::history::commit_history(options);
        }
        SubCommand::Login => config::login().await,
    }

    view::clean_up();
}
