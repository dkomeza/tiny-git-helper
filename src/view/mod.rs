use crossterm::{
    cursor::MoveToNextLine,
    execute,
    style::{Attribute, Color, Print, SetAttribute, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::stdout;

pub fn setup_view_controller() {
    enable_raw_mode().unwrap();

    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        disable_raw_mode().unwrap();
        println!();
        default_panic(panic_info);
    }));
}

pub fn clean_up() {
    disable_raw_mode().unwrap();
}

pub fn no_subcommand_error() {
    enable_raw_mode().unwrap();

    execute!(
        stdout(),
        SetForegroundColor(Color::Red),
        SetAttribute(Attribute::Bold),
        Print("error: "),
        SetAttribute(Attribute::Reset),
        Print("no subcommand provided\n\n"),
        MoveToNextLine(1),
        SetAttribute(Attribute::Bold),
        SetAttribute(Attribute::Underlined),
        Print("Usage:"),
        SetAttribute(Attribute::Reset),
        SetAttribute(Attribute::Bold),
        Print(" tgh"),
        SetAttribute(Attribute::Reset),
        Print(" [COMMAND]\n\n"),
        MoveToNextLine(1),
        Print("For more information try '"),
        SetAttribute(Attribute::Bold),
        Print("tgh --help"),
        SetAttribute(Attribute::Reset),
        Print("'\n"),
        MoveToNextLine(1),
    )
    .unwrap();

    disable_raw_mode().unwrap();
}
