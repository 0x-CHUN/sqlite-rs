use crate::constant::TABLE_MAX_PAGES;
use crate::page::Page;

pub struct Table {
    pub num_rows: usize,
    pub pages: Vec<Page>,
}

impl Table {
    pub fn new() -> Self {
        Table {
            num_rows: 0,
            pages: Vec::with_capacity(TABLE_MAX_PAGES),
        }
    }

    pub unsafe fn page_slot(&self, index: usize) -> *const Page {
        self.pages.as_ptr().offset(index as isize)
    }

    pub unsafe fn page_mut_slot(&mut self, index: usize) -> *mut Page {
        self.pages.as_mut_ptr().offset(index as isize)
    }
}
