use std::{io, process};

fn print_prompt() {
    println!("Sqlite-Rust > ")
}

fn read_line() -> String {
    let mut line = String::new();
    let bytes_read = io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    if bytes_read < 0 {
        panic!("Error Reading from input")
    }
    String::from(line.trim())
}

fn main() {
    loop {
        let cmd = read_line();
        if cmd.eq(".exit") {
            process::exit(0x0100);
        } else {
            println!("Unrecognized command {}", cmd);
        }
    }
}
