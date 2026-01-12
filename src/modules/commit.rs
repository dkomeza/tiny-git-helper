use clap::Parser;

mod functions;
mod views;

pub use views::commit_specific_files;

use crate::view::input::ReturnType;

#[derive(Parser)]
pub struct CommitOptions {
    /// Don't push changes to the remote
    #[clap(short, long)]
    pub no_push: bool,

    /// Don't use fancy commit message
    #[clap(long, conflicts_with = "force_fancy")]
    pub skip_fancy: bool,

    /// Force fancy commit message
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

pub fn commit_all_files(options: CommitOptions) {
    functions::is_valid_commit();

    let message = ask_commit_message(&options);

    match message {
        Ok(msg) => {
            functions::commit_all_files(msg, options.no_push);
        }
        Err(_) => {
            crate::out::print_error("Error getting commit message");
            std::process::exit(1);
        }
    }

    // commit_all_files(message, options.no_push);
}

fn ask_commit_message(options: &CommitOptions) -> Result<String, ReturnType> {
    use crate::view::input;

    if let Some(message) = &options.commit_message {
        return Ok(message.clone());
    }

    let config = crate::config::load_config();

    if options.force_fancy || (config.fancy && !options.skip_fancy) {
        // let labels = crate::config::utils::get_labels();
    }

    input::text("Enter commit message: ")
}
