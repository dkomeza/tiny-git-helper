use clap::Parser;

mod functions;
mod views;

pub use views::{commit_all_files, commit_menu, commit_specific_files};

#[derive(Parser)]
pub struct CommitOptions {
    /// Don't push changes to remote
    #[clap(short, long)]
    pub no_push: bool,

    /// Don't use fancy commit messages
    #[clap(long, conflicts_with = "force_fancy")]
    pub skip_fancy: bool,

    /// Force fancy commit messages
    #[clap(long, conflicts_with = "skip_fancy")]
    pub force_fancy: bool,

    /// Commit message (optional, skips the fancy commit message menu)
    pub commit_message: Option<String>,
}

impl Default for CommitOptions {
    fn default() -> Self {
        Self {
            no_push: false,
            skip_fancy: false,
            force_fancy: false,
            commit_message: None,
        }
    }
}
