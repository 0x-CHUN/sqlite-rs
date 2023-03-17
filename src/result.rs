use std::process;
use crate::constant::print_constants;
use crate::table::{db_close, Table};


#[derive(PartialEq, Debug)]
pub enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognized,
}

#[derive(PartialEq, Debug)]
pub enum PrepareResult {
    PrepareSuccess,
    PrepareUnrecognized,
    PrepareSyntaxErr,
    PrepareStringTooLong,
    PrepareInvalidId,
}

#[derive(PartialEq, Debug)]
pub enum ExecuteResult {
    ExecuteSuccess,
    ExecuteFail,
    ExecuteTableFull,
    ExecuteDuplicateKey,
}

pub fn get_meta_result(command: &str, table: &mut Table) -> MetaCommandResult {
    if command.eq(".exit") {
        db_close(table);
        process::exit(0x0100);
    } else if command.eq(".constants") {
        println!("Constants:");
        print_constants();
        return MetaCommandResult::MetaCommandSuccess;
    } else if command.eq(".btree") {
        println!("Btree:");
        table.print_tree();
        return MetaCommandResult::MetaCommandSuccess;
    }
    MetaCommandResult::MetaCommandUnrecognized
}