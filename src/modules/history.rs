use clap::{ArgGroup, Parser};

mod functions;
mod views;

pub use views::commit_history;

#[derive(Parser)]
#[clap(group(ArgGroup::new("fileGroup").args(&["file", "limit", "author", "branch", "all"]).conflicts_with("hash")))]
pub struct CommitHistoryOptions {
    /// Limit the number of commits to show
    #[clap(short, long, default_value = "10")]
    pub limit: Option<usize>,

    /// Author of the commits
    #[clap(short, long)]
    pub author: Option<String>,

    /// Branch to show commits from
    #[clap(short, long, conflicts_with = "all")]
    pub branch: Option<String>,

    /// Show commits from all branches
    #[clap(long, conflicts_with = "branch")]
    pub all: bool,

    /// Show the commits for a specific file (optional)
    #[clap(short, long)]
    pub file: Option<String>,

    /// Show the commit with the given hash
    #[clap(long)]
    pub hash: Option<String>,
}
