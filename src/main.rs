use clap::{Parser, Subcommand};

mod utils;
use utils::out;

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
    #[clap(name = "commit", about = "Commit changes to the repository")]
    #[clap(visible_alias = "cf")]
    CommitFiles(modules::commit::CommitOptions),
    #[clap(name = "ca", about = "Commit all files")]
    CommitAll(modules::commit::CommitOptions),
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    config::check_prerequisites().await;

    let subcmd = match args.subcmd {
        Some(subcmd) => subcmd,
        None => {
            view::no_subcommand_error();
            return;
        }
    };

    match subcmd {
        SubCommand::CommitAll(options) => {
            modules::commit::commit_all_files(options);
        }
        SubCommand::CommitFiles(options) => {
            modules::commit::commit_specific_files(options);
        }
    }

    view::clean_up();
}
