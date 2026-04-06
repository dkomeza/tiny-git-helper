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

    match ask_commit_message(&options) {
        Ok(msg) => {
            functions::commit_all_files(msg, options.no_push);
        }
        Err(err) => match err {
            ReturnType::Cancel => {
                return;
            }
            ReturnType::Exit => {
                std::process::exit(1);
            }
        },
    }
}

fn ask_commit_message(options: &CommitOptions) -> Result<String, ReturnType> {
    use crate::view::input;

    if let Some(message) = &options.commit_message {
        return Ok(message.clone());
    }

    let config = crate::config::load_config();

    if options.force_fancy || (config.fancy && !options.skip_fancy) {
        let mut message = String::new();
        let labels = crate::config::utils::get_labels();

        match input::list("Commit type: ", labels) {
            Ok(label) => {
                message.push_str(&label.emoji);
            }
            Err(err) => return Err(err),
        }

        match input::text("Commit message: ") {
            Ok(msg) => {
                message.push_str(&format!(" {}", msg));
            }
            Err(err) => return Err(err),
        }

        match input::text("Commit description (optional): ") {
            Ok(desc) => {
                if !desc.is_empty() {
                    message.push_str(&format!("\n\n{}", desc));
                }
            }
            Err(err) => return Err(err),
        }

        return Ok(message);
    }

    input::text("Enter commit message: ")
}
