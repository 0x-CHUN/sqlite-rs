use crate::constant::ROWS_PER_PAGE;
use crate::row::Row;

pub struct Page {
    rows: Vec<Row>,
}

impl Page {
    pub fn new() -> Self {
        Page {
            rows: Vec::with_capacity(ROWS_PER_PAGE)
        }
    }

    pub unsafe fn row_slot(&self, index: usize) -> *const Row {
        self.rows.as_ptr().offset(index as isize)
    }

    pub unsafe fn row_mut_slot(&mut self, index: usize) -> *mut Row {
        self.rows.as_mut_ptr().offset(index as isize)
    }
}