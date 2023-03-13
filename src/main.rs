mod result;
mod statement;
mod utils;

use std::{io, process};
use crate::result::{get_meta_result, MetaCommandResult};
use crate::result::PrepareResult::PrepareUnrecognized;
use crate::statement::{execute_statement, prepare_statement};
use crate::utils::{print_prompt, read_line};


fn main() {
    loop {
        print_prompt();
        let cmd = read_line();
        if cmd.starts_with(".") {
            let meta_result = get_meta_result(&cmd);
            match meta_result {
                MetaCommandResult::MetaCommandUnrecognized => {
                    println!("Unrecognized command {}", cmd);
                    continue;
                }
                MetaCommandResult::MetaCommandSuccess => continue
            }
        }
        let (stmt, prepare_result) = prepare_statement(&cmd);
        if prepare_result == PrepareUnrecognized {
            println!("Unrecognized keyword at start of {}.", cmd);
            continue;
        }
        execute_statement(&stmt);
        // println!("Executed.");
    }
}
