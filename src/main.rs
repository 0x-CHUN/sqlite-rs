#[warn(unused_assignments)]
use std::{env, process};
use crate::result::{get_meta_result, MetaCommandResult};
use crate::result::ExecuteResult::*;
use crate::result::PrepareResult::*;
use crate::statement::{execute_statement, prepare_statement};
use crate::table::{db_open};
use crate::utils::{print_prompt, read_line};

mod constant;
mod page;
mod pager;
mod table;
mod row;
mod statement;
mod result;
mod node;
mod cursor;
mod utils;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Must supply a database filename.");
        process::exit(0x0100);
    }
    let mut table = db_open(args[1].as_str());
    loop {
        print_prompt();
        let cmd = read_line();
        if cmd.starts_with(".") {
            let meta_result = get_meta_result(&cmd, &mut table);
            match meta_result {
                MetaCommandResult::MetaCommandUnrecognized => {
                    println!("Unrecognized command {}", cmd);
                    continue;
                }
                MetaCommandResult::MetaCommandSuccess => continue
            }
        }

        match prepare_statement(&cmd) {
            Ok(stmt) => {
                match execute_statement(stmt, &mut table) {
                    ExecuteSuccess => println!("Executed."),
                    ExecuteDuplicateKey => println!("Error: Duplicate key."),
                    ExecuteTableFull => println!("Error: Table full."),
                    _ => println!("Error: execute failed")
                }
            }
            Err(prepare_result) => {
                match prepare_result {
                    PrepareUnrecognized =>
                        println!("Unrecognized keyword at start of {}.", cmd),
                    PrepareSyntaxErr =>
                        println!("Syntax error. Could not parse statement."),
                    PrepareStringTooLong =>
                        println!("String is too long."),
                    PrepareInvalidId =>
                        println!("ID must be positive."),
                    _ => {}
                };
                continue;
            }
        };
    }
}
