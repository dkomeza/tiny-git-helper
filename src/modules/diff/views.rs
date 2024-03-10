use super::functions::*;

pub fn show_diff(options: super::DiffOptions) {
    use crate::functions::is_git_repo;
    use crate::out;

    if !is_git_repo() {
        out::print_error("Not a git repository (or any of the parent directories)");
        return;
    }

    let files = get_diff_files();

    println!("Files: {:?}", files);
}
