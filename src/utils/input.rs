pub fn text(message: &str) -> String {
    use std::io::{stdin, stdout, Write};

    let mut input = String::new();

    print!("{}", message);
    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("Failed to read input");

    return input.trim().to_string();
}
