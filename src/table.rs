use std::process;
use crate::constant::{INTERNAL_NODE_CELL_SIZE, INTERNAL_NODE_MAX_CELLS};
use crate::node::NodeType;
use crate::node::NodeType::NodeLeaf;
use crate::page::Page;
use crate::pager::{Pager, pager_open};

pub struct Table {
    pub(crate) root_page_num: usize,
    pub(crate) pager: Pager,
}

impl Table {
    pub(crate) fn new(pager: Pager) -> Self {
        Table {
            pager,
            root_page_num: 0,
        }
    }

    pub fn find(&self, key: u32) -> (usize, usize) {
        let root_page_num = self.root_page_num;
        let page = self.pager.get_page_view(root_page_num);
        if page.is_none() {
            return (0, 0);
        }
        self.find_by_page(page.unwrap(), key, root_page_num)
    }

    fn find_by_page_num(&self, page_num: usize, key: u32) -> (usize, usize) {
        let page = self.pager.get_page_view(page_num);
        if page.is_none() {
            println!("page {} not exist", page_num);
            process::exit(0x0010);
        }
        self.find_by_page(page.unwrap(), key, page_num)
    }

    fn find_by_page(&self, page: &Page, key: u32, page_num: usize) -> (usize, usize) {
        if *page.get_node_type() == NodeLeaf {
            self.leaf_node_find(page, key, page_num)
        } else {
            self.internal_node_find(page, key)
        }
    }

    pub fn internal_node_find(&self, page: &Page, key: u32) -> (usize, usize) {
        let cell_index = page.internal_node_find_child(key);
        if page.get_internal_node_key(cell_index) >= key {
            let child_page_num = page.get_internal_node_child(cell_index);
            return self.find_by_page_num(child_page_num, key);
        }
        let right_child_num = page.get_internal_node_right_child();
        self.find_by_page_num(right_child_num, key)
    }

    fn leaf_node_find(&self, page: &Page, key: u32, page_num: usize) -> (usize, usize) {
        (page_num, page.leaf_node_find(key))
    }

    pub fn internal_node_insert(&mut self, parent_page_num: usize, child_page_num: usize) {
        let child_max_key = self.pager.get_page_view(child_page_num).unwrap()
            .get_node_max_key();

        let parent = self.pager.get_page(parent_page_num);
        let right_child_page_num = parent.get_internal_node_right_child();
        let child_max_key_index = parent.internal_node_find_child(child_max_key);
        let origin_num_keys = parent.get_internal_node_num_keys();
        if origin_num_keys >= INTERNAL_NODE_MAX_CELLS {
            println!("Need to implement splitting internal node");
            process::exit(0x0010);
        }
        parent.increase_internal_node_num_keys(1);
        let right_child_max_key = self.pager.get_page_view(right_child_page_num).unwrap()
            .get_node_max_key();

        if child_max_key > right_child_max_key {
            let parent = self.pager.get_page(parent_page_num);
            parent.set_internal_node_right_child(child_page_num);
            parent.set_internal_node_child(origin_num_keys, right_child_page_num);
            parent.set_internal_node_key(origin_num_keys, right_child_max_key);
        } else {
            let parent = self.pager.get_page(parent_page_num);
            for i in (child_max_key_index + 1..=origin_num_keys).rev() {
                unsafe {
                    std::ptr::copy_nonoverlapping(parent.leaf_node_cell(i - 1),
                                                  parent.leaf_node_cell(i) as *mut u8,
                                                  INTERNAL_NODE_CELL_SIZE);
                }
            }
            parent.set_internal_node_child(child_max_key_index, child_page_num);
            parent.set_internal_node_key(child_max_key_index, child_max_key);
        }
    }

    pub fn print_tree(&self) {
        fn print_tree_node(pager: &Pager, page_num: usize, indentation_level: usize) {
            fn indent(level: usize) {
                (0..level).for_each(|_| print!(" "));
            }
            let node = pager.get_page_view(page_num);
            match node {
                Some(page) => {
                    match page.get_node_type() {
                        NodeType::NodeLeaf => {
                            let num_keys = page.leaf_node_num_cells();
                            indent(indentation_level);
                            println!("- leaf (size {})", num_keys);
                            for i in 0..num_keys {
                                indent(indentation_level + 1);
                                println!("{}", page.leaf_node_key(i));
                            }
                        }
                        NodeType::NodeInternal => {
                            let num_keys = page.get_internal_node_num_keys();
                            indent(indentation_level);
                            println!("- internal (size {})", num_keys);
                            for i in 0..num_keys {
                                let child = page.get_internal_node_child(i);
                                print_tree_node(pager, child, indentation_level + 1);
                                indent(indentation_level + 1);
                                println!("- key {}", page.get_internal_node_key(i));
                            }
                            let child = page.get_internal_node_right_child();
                            print_tree_node(pager, child, indentation_level + 1);
                        }
                    }
                }
                _ => ()
            }
        }
        print_tree_node(&self.pager, 0, 0);
    }
}

pub fn db_open(file_name: &str) -> Table {
    let pager = pager_open(file_name);
    Table::new(pager)
}

pub fn db_close(table: &mut Table) {
    for i in 0..table.pager.num_pages {
        table.pager.pager_flush(i);
    }
}