use std::io;
use std::io::Write;
use std::iter::Rev;
use std::ops::Range;
use std::process::exit;
use crate::constant::{EMAIL_OFFSET, EMAIL_SIZE, LEAF_NODE_CELL_SIZE, LEAF_NODE_LEFT_SPLIT_COUNT, USERNAME_OFFSET, USERNAME_SIZE};
use crate::page::Page;
use crate::row::Row;

pub fn print_prompt() {
    print!("Sqlite-rs >");
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
    if bytes_read <= 0 {
        panic!("Error Reading from input")
    }
    String::from(line.trim())
}

pub unsafe fn serialize_row(cell: *mut u8, source: &Row) {
    std::ptr::write(cell as *mut u32, source.id);

    std::ptr::write((cell as usize + USERNAME_OFFSET) as *mut [u8; USERNAME_SIZE], [0 as u8; USERNAME_SIZE]);
    std::ptr::copy(source.username.as_ptr(), (cell as usize + USERNAME_OFFSET) as *mut u8, source.username.len());

    std::ptr::write((cell as usize + EMAIL_OFFSET) as *mut [u8; EMAIL_SIZE], [0 as u8; EMAIL_SIZE]);
    std::ptr::copy(source.email.as_ptr(), (cell as usize + EMAIL_OFFSET) as *mut u8, source.email.len());
}

pub fn copy_page_data(rang: Rev<Range<usize>>, src_ptr: *const Page, dst_page: &mut Page, key: u32, value: &Row, value_cell_num: usize) {
    for i in rang {
        let index_within_node = i % LEAF_NODE_LEFT_SPLIT_COUNT;
        let destination = dst_page.leaf_node_cell(index_within_node);
        unsafe {
            if i == value_cell_num {
                dst_page.set_leaf_node_key(index_within_node, key);
                let destination = dst_page.leaf_node_value(index_within_node);
                serialize_row(destination as *mut u8, value);
            } else if i > value_cell_num {
                std::ptr::copy((*src_ptr).leaf_node_cell(i - 1), destination as *mut u8, LEAF_NODE_CELL_SIZE);
            } else {
                std::ptr::copy((*src_ptr).leaf_node_cell(i), destination as *mut u8, LEAF_NODE_CELL_SIZE)
            }
        }
    }
}
