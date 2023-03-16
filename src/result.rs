use std::process;

#[derive(PartialEq, Debug)]
pub enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognized,
}

#[derive(PartialEq, Debug)]
pub enum PrepareResult {
    PrepareUnrecognized,
    PrepareSyntaxErr,
    PrepareStringTooLong,
    PrepareInvalidId,
}

pub fn get_meta_result(command: &str) -> MetaCommandResult {
    if command.eq(".exit") {
        process::exit(0x0100);
    }
    MetaCommandResult::MetaCommandUnrecognized
}

#[derive(PartialEq, Debug)]
pub enum ExecuteResult {
    ExecuteSuccess,
    ExecuteFail,
    ExecuteTableFull,
}
