use clap::Parser;

mod functions;
mod views;

pub use views::show_diff;

#[derive(Parser)]
pub struct DiffOptions {}
