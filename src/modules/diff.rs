use clap::Parser;

mod functions;
mod views;

pub use views::show_diff;

#[derive(Parser)]
pub struct DiffOptions {
    /// Show the extended diff
    #[clap(short, long)]
    pub extended: bool,

    /// The file to show the diff for
    #[clap(short, long, conflicts_with = "selector")]
    pub file: Option<String>,

    /// Show the file selector
    #[clap(short, long, conflicts_with = "file")]
    pub selector: bool,
}
