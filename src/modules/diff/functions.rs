pub fn get_diff_files() -> Vec<String> {
    use crate::out;
    use std::process::Command;

    let output = Command::new("git")
        .arg("diff")
        .arg("--name-only")
        .output()
        .expect("Failed to execute git diff");

    let files = String::from_utf8_lossy(&output.stdout);

    if files.is_empty() {
        return vec![];
    }

    let files: Vec<&str> = files.split('\n').collect();
    let files: Vec<String> = files
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    files
}

pub fn is_file_in_diff(file: &str) -> bool {
    let files = get_diff_files();
    files.contains(&file.to_string())
}

pub fn validate_file_selection(
    files: &[inquire::list_option::ListOption<&String>],
) -> Result<inquire::validator::Validation, inquire::CustomUserError> {
    if files.len() == 0 {
        return Ok(inquire::validator::Validation::Invalid(
            "You must select at least one file".into(),
        ));
    }

    Ok(inquire::validator::Validation::Valid)
}
