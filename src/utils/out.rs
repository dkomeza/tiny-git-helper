pub fn print_error(message: &str) {
    println!("\x1B[1;31m{}\x1B[m", message);
}

pub fn print_success(message: &str) {
    println!("\x1B[1;32m{}\x1B[m", message);
}