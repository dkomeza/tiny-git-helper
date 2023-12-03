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

pub fn password(message: &str, required: bool) -> String {
    use crate::out;
    use std::io::{stdin, stdout, Write};

    let mut input = String::new();

    print!("{}\x1B[8m", message);

    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("Failed to read input");

    print!("\x1B[28m");

    if required && input.trim().len() == 0 {
        out::print_error("This field is required.\n");
        return password(message, required);
    }

    return input.trim().to_string();
}

pub fn confirm(message: &str, default: bool) -> bool {
    use crate::out;
    use std::io::{stdin, stdout, Write};

    let mut input = String::new();

    print!("{} [{}]: ", message, if default { "Y/n" } else { "y/N" });
    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("Failed to read input");

    if input.trim().len() == 0 {
        return default;
    }

    return match input.trim().to_lowercase().as_str() {
        "y" | "yes" => true,
        "n" | "no" => false,
        _ => {
            out::print_error("Invalid input.\n");
            confirm(message, default)
        }
    };
}