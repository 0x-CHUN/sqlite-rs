use std::cell::RefCell;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::process;
use crate::constant::{PAGE_SIZE, TABLE_MAX_PAGES};
use crate::page::Page;

pub struct Pager {
    file_descriptor: RefCell<File>,
    pages: Vec<Option<Box<Page>>>,
    pub(crate) num_pages: usize,
}

impl Pager {
    pub(crate) fn new(file: File) -> Self {
        fn num_pages_file(file_length: u64) -> usize {
            let num_page = file_length / PAGE_SIZE as u64;
            if file_length % PAGE_SIZE as u64 != 0 {
                println!("Database file is not a whole number of pages. Corrupt file.");
                process::exit(0x0100);
            }
            num_page as usize
        }
        Pager {
            num_pages: num_pages_file(file.metadata().unwrap().len()),
            file_descriptor: RefCell::new(file),
            pages: std::iter::repeat_with(|| None).take(TABLE_MAX_PAGES).collect::<Vec<_>>(),
        }
    }

    pub(crate) fn get_page_view(&self, page_num: usize) -> Option<&Page> {
        if page_num > TABLE_MAX_PAGES {
            panic!("Tried to fetch page number out of bounds. {} > {}", page_num, TABLE_MAX_PAGES);
        }

        unsafe {
            let ptr = self.pages.as_ptr();
            let page = ptr.offset(page_num as isize);
            if (*page).is_none() {
                self.load_page(page_num);
            }
            let page = ptr.offset(page_num as isize);
            Some((*page).as_ref().unwrap().as_ref())
        }
    }

    fn load_page(&self, page_num: usize) {
        let mut new_page = Page::new();
        if page_num <= self.num_pages {
            self.file_descriptor.borrow_mut().seek(SeekFrom::Start(page_num as u64 * PAGE_SIZE as u64))
                .expect("Pager load page error");
            let result = self.file_descriptor.borrow_mut().read(&mut new_page.buf);
            if result.is_err() {
                println!("Error reading file: {}", result.unwrap());
                process::exit(0x0100);
            }
        }

        unsafe {
            let ptr = self.pages.as_ptr();
            let pages = ptr as *mut Option<Box<Page>>;
            (*pages.offset(page_num as isize)) = Some(Box::new(new_page));
        }
    }

    pub(crate) fn get_page(&mut self, page_num: usize) -> &mut Page {
        if page_num > TABLE_MAX_PAGES {
            panic!("Tried to fetch page number out of bounds. {} > {}", page_num, TABLE_MAX_PAGES);
        }
        unsafe {
            let ptr = self.pages.as_ptr();
            let page = ptr.offset(page_num as isize);
            if (*page).is_none() {
                self.load_page(page_num);
                if page_num >= self.num_pages {
                    self.num_pages += 1;
                }
            }
        }
        let pages = self.pages.as_mut_ptr();
        unsafe {
            let page = pages.offset(page_num as isize);
            (*page).as_mut().unwrap().as_mut()
        }
    }

    pub fn get_leftmost_leaf_page_num(&self, page_num: usize) -> usize {
        let page = self.get_page_view(page_num);
        if page.is_none() {
            panic!("invalid page number {}", page_num);
        }
        let p = page.unwrap();
        if p.is_leaf_node() {
            return page_num;
        }
        let child_page_num = p.get_internal_node_child(0);
        return self.get_leftmost_leaf_page_num(child_page_num);
    }

    pub fn pager_flush(&mut self, page_num: usize) {
        match &self.pages[page_num] {
            Some(page) => {
                self.file_descriptor.borrow_mut().seek(SeekFrom::Start(page_num as u64 * PAGE_SIZE as u64))
                    .expect("Pager flush :seek error");
                self.file_descriptor.borrow_mut().write(page.buf.as_slice())
                    .expect("Pager flush : write error");
                self.file_descriptor.borrow_mut().flush()
                    .expect("Pager flush : flush error");
            }
            None => ()
        }
    }

    fn close(&mut self) {
        self.file_descriptor.borrow_mut().flush().expect("Pager close error");
    }

    pub(crate) fn get_unused_page_num(&self) -> usize {
        self.num_pages
    }
}

pub fn pager_open(file_name: &str) -> Pager {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open(file_name)
        .unwrap();

    let mut pager = Pager::new(file);
    if pager.num_pages == 0 {
        let root_node = pager.get_page(0);
        root_node.initialize_leaf_node();
        root_node.set_node_root(true);
    }
    pager
}