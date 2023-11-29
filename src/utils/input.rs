pub fn text(message: &str, required: bool) -> String {
    use crate::out;
    use std::io::{stdin, stdout, Write};

    let mut input = String::new();

    print!("{}", message);
    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("Failed to read input");

    if required && input.trim().len() == 0 {
        out::print_error("This field is required.\n");
        return text(message, required);
    }

    return input.trim().to_string();
}
