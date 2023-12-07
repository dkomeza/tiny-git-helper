use std::env;

mod utils;
use utils::out;
use utils::out::clear_screen;

mod config;
mod functions;
mod views;

fn setup_ui() {
    use inquire::ui::{Attributes, Color, RenderConfig, StyleSheet};

    let mut render_config = RenderConfig::default();
    if config::load_config().color != config::defines::COLOR::NORMAL {
        let color = config::get_inquire_color();
        render_config.prompt = StyleSheet::new().with_fg(color);
    }
    render_config.answer = StyleSheet::new()
        .with_fg(Color::Grey)
        .with_attr(Attributes::BOLD);
    render_config.help_message = StyleSheet::new().with_fg(Color::DarkGrey);

    inquire::set_global_render_config(render_config);
}

struct Args {
    mode: String,
    args: Vec<String>,
}
impl Args {
    fn new() -> Args {
        let args: Vec<String> = env::args().skip(1).collect();

        if args.len() == 0 {
            return Args {
                mode: String::from(""),
                args: vec![],
            };
        }

        let mode = args[0].clone();

        if args.len() == 1 {
            return Args { mode, args: vec![] };
        }

        let args = args[1..].to_vec();

        Args { mode, args }
    }
}

#[tokio::main]
async fn main() {
    let args = Args::new();

    config::check_prerequisites().await;

    setup_ui();

    if args.mode.len() == 0 {
        views::menu();
        return;
    }

    match args.mode.as_str() {
        "commit" => {
            views::commit::commit_menu();
        }
        "login" => {
            let _ = config::login().await;
        }
        "help" => views::help::print_help(false, args),
        "version" => views::help::print_version(),
        _ => {
            views::help::print_help(true, args);
        }
    };
}
