use crate::constant::{EMAIL_SIZE, USERNAME_SIZE};
use crate::cursor::Cursor;
use crate::result::{ExecuteResult, PrepareResult};
use crate::result::ExecuteResult::*;
use crate::result::PrepareResult::*;
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
        StatementType::StatementSelect => execute_select(table),
        _ => ExecuteFail
    }
}

fn execute_insert(statement: &Statement, table: &mut Table) -> ExecuteResult {
    match statement.row_to_insert.as_ref() {
        Some(row_to_insert) => {
            let (page_num, cell_num) = table.find(row_to_insert.id);
            let page = table.pager.get_page(page_num);
            if cell_num < page.leaf_node_num_cells() {
                let key_at_index = page.leaf_node_key(cell_num);
                if key_at_index == row_to_insert.id {
                    return ExecuteDuplicateKey;
                }
            }
            let mut cursor = Cursor {
                table,
                page_num,
                cell_num,
                end_of_table: false,
            };
            unsafe { cursor.leaf_node_insert((*row_to_insert).id, row_to_insert) };
            ExecuteSuccess
        }
        _ => ExecuteFail
    }
}

fn execute_select(table: &mut Table) -> ExecuteResult {
    let mut cursor = Cursor::table_start(table);
    while !cursor.end_of_table {
        let row = cursor.cursor_value();
        println!("{}, {}, {}", (*row).id, (*row).username, (*row).email);
        cursor.advance();
    }
    ExecuteSuccess
}