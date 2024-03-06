use clap::Parser;

mod functions;
mod views;

pub use views::clone_menu;

#[derive(Parser)]
pub struct CloneOptions {
    /// Search public repositories
    #[clap(short, long)]
    pub public: bool,
}

impl Default for CloneOptions {
    fn default() -> Self {
        Self { public: false }
    }
}
