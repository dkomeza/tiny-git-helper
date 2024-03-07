use clap::Parser;

mod functions;
mod views;

pub use views::commit_history;

#[derive(Parser)]
pub struct CommitHistoryOptions {
    /// Limit the number of commits to show
    #[clap(short, long, default_value = "10")]
    pub limit: Option<usize>,

    /// Author of the commits
    #[clap(short, long)]
    pub author: Option<String>,

    /// Branch to show commits from
    #[clap(short, long)]
    pub branch: Option<String>,

    /// Show the commits for a specific file (optional)
    pub file: Option<String>,
}
