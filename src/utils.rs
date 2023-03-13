use std::io;
use std::io::Write;
use std::process::exit;

pub fn print_prompt() {
    print!("Sqlite-rs > ");
    if io::stdout().flush().is_err() {
        println!("Stdout Error");
        exit(0x0100);
    }
}

pub fn read_line() -> String {
    let mut line = String::new();
    let bytes_read = io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    if bytes_read < 0 {
        panic!("Error Reading from input")
    }
    String::from(line.trim())
}