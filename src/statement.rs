use crate::result::PrepareResult;

#[derive(PartialEq)]
pub enum StatementType {
    StatementInsert,
    StatementSelect,
    StatementUnsupported,
}

pub struct Statement {
    stmt_type: StatementType,
}

pub fn get_stmt_type(command: &str) -> StatementType {
    if command.starts_with("insert") {
        StatementType::StatementInsert
    } else if command.starts_with("select") {
        StatementType::StatementSelect
    } else {
        StatementType::StatementUnsupported
    }
}

pub fn prepare_statement(command: &str) -> (Box<Statement>, PrepareResult) {
    let stmt_type = get_stmt_type(command);
    let stmt = Statement {
        stmt_type
    };
    match stmt.stmt_type {
        StatementType::StatementUnsupported =>
            (Box::new(stmt), PrepareResult::PrepareUnrecognized),
        _ => (Box::new(stmt), PrepareResult::PrepareSuccess)
    }
}

pub fn execute_statement(stmt: &Statement) {
    match &stmt.stmt_type {
        StatementType::StatementInsert => {
            println!("Insert")
        }
        StatementType::StatementSelect => {
            println!("Select")
        }
        _ => {
            println!("Error")
        }
    }
}