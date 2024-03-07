pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Black,
}

pub fn print_error(message: &str) {
    println!("{}", format_error(message));
}

pub fn print_success(message: &str) {
    println!("{}", format_success(message));
}

pub fn clear_screen() {
    print!("{}[2J", 27 as char);
    println!("{}[H", 27 as char);
}

pub fn print_dim(message: &str) {
    println!("{}", format_dim(message));
}

pub fn print_bold(message: &str) {
    println!("{}", format_bold(message));
}

pub fn format_error(message: &str) -> String {
    format!("\x1B[1;31m{}\x1B[m", message)
}

pub fn format_success(message: &str) -> String {
    format!("\x1B[1;32m{}\x1B[m", message)
}

pub fn format_dim(message: &str) -> String {
    format!("\x1B[2m{}\x1B[m", message)
}

pub fn format_bold(message: &str) -> String {
    format!("\x1B[1m{}\x1B[m", message)
}

pub fn format_color(message: &str, color: Color) -> String {
    let color = match color {
        Color::Red => 31,
        Color::Green => 32,
        Color::Yellow => 33,
        Color::Blue => 34,
        Color::Magenta => 35,
        Color::Cyan => 36,
        Color::White => 37,
        Color::Black => 30,
    };

    format!("\x1B[{}m{}\x1B[m", color, message)
}
