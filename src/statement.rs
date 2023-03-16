use crate::constant::{EMAIL_SIZE, ROWS_PER_PAGE, TABLE_MAX_ROWS, USERNAME_SIZE};
use crate::result::{ExecuteResult, PrepareResult};
use crate::result::PrepareResult::{PrepareInvalidId, PrepareStringTooLong, PrepareSyntaxErr, PrepareUnrecognized};
use crate::row::Row;
use crate::table::Table;

#[derive(PartialEq)]
pub enum StatementType {
    StatementInsert,
    StatementSelect,
    StatementUnsupported,
}

pub struct Statement {
    stmt_type: StatementType,
    row_to_insert: Option<Row>,
}


pub fn prepare_statement(command: &str) -> Result<Box<Option<Statement>>, PrepareResult> {
    if command.starts_with("insert") {
        let args: Vec<&str> = command.split(" ").collect();
        if args.len() < 4 {
            return Err(PrepareSyntaxErr);
        }
        let id = match args[1].trim().parse::<u32>() {
            Ok(id) => id,
            Err(_) => return Err(PrepareInvalidId),
        };
        let username = if args[2].trim().len() < USERNAME_SIZE {
            String::from(args[2].trim())
        } else {
            return Err(PrepareStringTooLong);
        };
        let email = if args[3].trim().len() < EMAIL_SIZE {
            String::from(args[3].trim())
        } else {
            return Err(PrepareStringTooLong);
        };
        Ok(Box::new(Some(Statement {
            stmt_type: StatementType::StatementInsert,
            row_to_insert: Some(Row {
                id,
                username,
                email,
            }),
        })))
    } else if command.starts_with("select") {
        Ok(Box::new(Some(Statement {
            stmt_type: StatementType::StatementSelect,
            row_to_insert: None,
        })))
    } else {
        Err(PrepareUnrecognized)
    }
}

pub fn execute_statement(statement: Box<Option<Statement>>, table: &mut Table) -> ExecuteResult {
    let stmt = statement.unwrap();
    match &stmt.stmt_type {
        StatementType::StatementInsert => execute_insert(&stmt, table),
        StatementType::StatementSelect => execute_select(&stmt, table),
        _ => ExecuteResult::ExecuteFail
    }
}

unsafe fn row_mut_slot(table: &mut Table, row_num: usize) -> *mut Row {
    let page = table.page_mut_slot(row_num / ROWS_PER_PAGE);
    (*page).row_mut_slot(row_num % ROWS_PER_PAGE)
}

unsafe fn row_slot(table: &mut Table, row_num: usize) -> *const Row {
    let page = table.page_slot(row_num / ROWS_PER_PAGE);
    if page.is_null() {
        return std::ptr::null();
    }
    (*page).row_slot(row_num % ROWS_PER_PAGE)
}

fn execute_insert(stmt: &Statement, table: &mut Table) -> ExecuteResult {
    match stmt.row_to_insert.as_ref() {
        Some(row_to_insert) => {
            if table.num_rows > TABLE_MAX_ROWS {
                return ExecuteResult::ExecuteTableFull;
            }
            unsafe {
                let row = row_mut_slot(table, table.num_rows);
                std::ptr::write(row, Row {
                    id: (*row_to_insert).id,
                    username: String::from((*row_to_insert).username.as_str()),
                    email: String::from((*row_to_insert).email.as_str()),
                });
            }
            table.num_rows += 1;
            ExecuteResult::ExecuteSuccess
        }
        _ => ExecuteResult::ExecuteFail
    }
}

fn execute_select(stmt: &Statement, table: &mut Table) -> ExecuteResult {
    for i in 0..table.num_rows {
        unsafe {
            let row = row_slot(table, i);
            println!("{}, {}, {}", (*row).id, (*row).username, (*row).email)
        }
    }
    ExecuteResult::ExecuteSuccess
}
