use std::env;

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

struct Args {
    mode: String,
    args: Vec<String>,
    help: bool,
}
impl Args {
    fn new() -> Args {
        let args: Vec<String> = env::args().skip(1).collect();

        if args.len() == 0 {
            return Args {
                mode: String::from(""),
                args: vec![],
                help: false,
            };
        }

        let mode = args[0].clone();

        if args.len() == 1 {
            return Args { mode, args: vec![], help: false };
        }

        let mut args = args[1..].to_vec();

        let mut help = false;
        for i in 0..args.len() {
            if args[i] == "--help" || args[i] == "-h" {
                help = true;
                args.remove(i);
                break;
            }
        }

        Args { mode, args, help }
    }
}

#[tokio::main]
async fn main() {
    let args = Args::new();

    config::check_prerequisites().await;

    setup_ui();

    if args.help && args.mode.len() == 0 {
        modules::help::print_help(false, args);
        return;
    }

    if args.mode.len() == 0 {
        modules::menu();
        return;
    }

    match args.mode.as_str() {
        "commit" | "c" | "ca" | "cf" => {
            modules::commit::handle_commit(args);
        }
        "login" => {
            let _ = config::login().await;
        }
        "version" => modules::help::print_version(),

        _ => {
            modules::help::print_help(true, args);
        }
    };
}
