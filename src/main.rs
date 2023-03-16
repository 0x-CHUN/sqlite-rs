mod result;
mod statement;
mod utils;
mod row;
mod constant;
mod page;
mod table;

use std::{io, process};
use crate::result::{get_meta_result, MetaCommandResult, PrepareResult};
use crate::result::PrepareResult::*;
use crate::statement::{execute_statement, prepare_statement};
use crate::table::Table;
use crate::utils::{print_prompt, read_line};


fn main() {
    let mut table = Table::new();
    loop {
        print_prompt();
        let cmd = read_line();
        if cmd.starts_with(".") {
            let meta_result = get_meta_result(&cmd);
            match meta_result {
                MetaCommandResult::MetaCommandUnrecognized => {
                    println!("Unrecognized command : {}", cmd);
                    continue;
                }
                MetaCommandResult::MetaCommandSuccess => continue
            }
        }
        match prepare_statement(&cmd) {
            Ok(stmt) => execute_statement(stmt, &mut table),
            Err(e) => {
                match e {
                    PrepareUnrecognized =>
                        println!("Unrecognized command : {}.", cmd),
                    PrepareSyntaxErr =>
                        println!("Syntax error : {}.", cmd),
                    PrepareStringTooLong =>
                        println!("Too long string"),
                    PrepareInvalidId =>
                        println!("Invalid id"),
                };
                continue;
            }
        };
        println!("Executed.")
    }
}
