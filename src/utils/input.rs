pub struct Option {
    pub title: String,
    pub id: i8,
}

impl Option {
    pub fn new(title: &str, id: i8) -> Option {
        return Option {
            title: title.to_string(),
            id,
        };
    }
}


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

pub fn list(message: &str, options: Vec<Option>) -> usize {
    use crate::out;
    use std::io::{stdin, stdout, Write};

    let mut input = String::new();

    print!("{}\n", message);

    for (i, option) in options.iter().enumerate() {
        println!("    [{}] {}", i + 1, option.title);
    }

    print!("> ");
    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("Failed to read input");

    let index = match input.trim().parse::<usize>() {
        Ok(i) => i - 1,
        Err(_) => {
            out::print_error("Invalid input.\n");
            return list(message, options);
        }
    };

    if index >= options.len() {
        out::print_error("Invalid input.\n");
        return list(message, options);
    }

    return index;
}
