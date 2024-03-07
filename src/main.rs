use clap::{Parser, Subcommand};

mod utils;
use utils::out;
use utils::out::clear_screen;

mod config;
mod functions;
mod modules;

fn setup_ui() {
    use inquire::ui::{Attributes, Color, RenderConfig, StyleSheet};

    let mut render_config = RenderConfig::default();
    if config::load_config().color != config::defines::COLOR::NORMAL {
        render_config.prompt =
            StyleSheet::new().with_fg(config::load_config().color.as_inquire_color());
    }
    render_config.answer = StyleSheet::new()
        .with_fg(Color::Grey)
        .with_attr(Attributes::BOLD);
    render_config.help_message = StyleSheet::new().with_fg(Color::DarkGrey);

    inquire::set_global_render_config(render_config);
}

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

    #[clap(name = "login", about = "Login to GitHub")]
    Login,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    config::check_prerequisites().await;
    setup_ui();

    let subcmd = match args.subcmd {
        Some(subcmd) => subcmd,
        None => {
            out::print_error("\nNo subcommand provided\n");
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
        SubCommand::Login => {
            return config::login().await;
        }
    }
}
