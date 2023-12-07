pub fn print_error(message: &str) {
    println!("\x1B[1;31m{}\x1B[m", message);
}

pub fn print_success(message: &str) {
    println!("\x1B[1;32m{}\x1B[m", message);
}

pub fn clear_screen() {
    print!("{}[2J", 27 as char);
    println!("{}[H", 27 as char);
}

pub fn print_dim(message: &str) {
    println!("\x1B[2m{}\x1B[m", message);
}

pub fn print_bold(message: &str) {
    println!("\x1B[1m{}\x1B[m", message);
}
