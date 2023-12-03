pub fn print_error(message: &str) {
    println!("\x1B[1;31m{}\x1B[m", message);
}

pub fn print_warning(message: &str) {
    println!("\x1B[1;33m{}\x1B[m", message);
}

pub fn print_success(message: &str) {
    println!("\x1B[1;32m{}\x1B[m", message);
}

pub fn color(message: &str, color: crate::config::defines::COLOR) -> String {
    let color = match color {
        crate::config::defines::COLOR::NORMAL => 0,
        crate::config::defines::COLOR::RED => 31,
        crate::config::defines::COLOR::GREEN => 32,
        crate::config::defines::COLOR::YELLOW => 33,
        crate::config::defines::COLOR::BLUE => 34,
        crate::config::defines::COLOR::MAGENTA => 35,
        crate::config::defines::COLOR::CYAN => 36,
        crate::config::defines::COLOR::WHITE => 37,
        crate::config::defines::COLOR::GRAY => 90,
    };

    return format!("\x1B[{}m{}\x1B[m", color, message);
}
