use crate::constant::*;
use crate::page::Page;
use crate::row::Row;
use crate::table::Table;
use crate::utils::{copy_page_data, serialize_row};

#[warn(unused_assignments)]
pub struct Cursor<'a> {
    pub(crate) table: &'a mut Table,
    pub(crate) page_num: usize,
    pub(crate) cell_num: usize,
    pub(crate) end_of_table: bool,
}

impl<'a> Cursor<'a> {
    pub fn table_start(table: &'a mut Table) -> Self {
        let root_page_num = table.root_page_num;

        let leaf_page_num = table.pager.get_leftmost_leaf_page_num(root_page_num);
        let leaf_node = table.pager.get_page_view(leaf_page_num).unwrap();
        let num_cells = leaf_node.leaf_node_num_cells();

        Cursor {
            table,
            cell_num: 0,
            page_num: leaf_page_num,
            end_of_table: num_cells == 0,
        }
    }

    pub fn get_page(&mut self) -> &mut Page {
        self.table.pager.get_page(self.page_num)
    }

    pub fn get_page_view(&self) -> Option<&Page> {
        self.table.pager.get_page_view(self.page_num)
    }

    pub fn advance(&mut self) {
        let page = self.table.pager.get_page_view(self.page_num).unwrap();
        self.cell_num += 1;
        if self.cell_num >= page.leaf_node_num_cells() {
            let next_page_num = page.get_leaf_node_next_leaf();
            if next_page_num == 0 {
                self.end_of_table = true;
            } else {
                self.page_num = next_page_num;
                self.cell_num = 0;
            }
        }
    }

    pub fn cursor_value(&mut self) -> Box<Row> {
        let cell_num = self.cell_num;
        let page = self.get_page_view().unwrap();
        unsafe { page.row_mut_slot(cell_num) }
    }

    pub unsafe fn leaf_node_insert(&mut self, key: u32, value: &Row) {
        let cell_num = self.cell_num;
        let page = self.get_page();
        let num_cells = page.leaf_node_num_cells();
        if num_cells >= LEAF_NODE_MAX_CELLS {
            self.leaf_node_split_and_insert(value.id, value);
            return;
        }
        if cell_num < num_cells {
            for i in (cell_num + 1..=num_cells).rev() {
                std::ptr::copy_nonoverlapping(page.leaf_node_cell(i - 1),
                                              page.leaf_node_cell(i) as *mut u8,
                                              LEAF_NODE_CELL_SIZE);
            }
        }
        page.set_leaf_node_num_cells(num_cells + 1);
        page.set_leaf_node_key(cell_num, key);

        let cell = page.leaf_node_value(cell_num);
        serialize_row(cell, value);
    }

    fn leaf_node_split_and_insert(&mut self, key: u32, value: &Row) {
        let value_cell_num = self.cell_num;
        let new_page_num = self.table.pager.get_unused_page_num();
        let old_max;
        {
            let old_node = self.get_page_view().unwrap();
            old_max = old_node.get_node_max_key();
            let old_next_page_num = old_node.get_leaf_node_next_leaf();
            let old_node_parent_num = old_node.get_node_parent();
            let old_node_ptr = old_node as *const Page;
            let new_node = self.table.pager.get_page(new_page_num);
            new_node.initialize_leaf_node();
            new_node.set_node_parent(old_node_parent_num);
            new_node.set_leaf_node_next_leaf(old_next_page_num);
            copy_page_data((LEAF_NODE_LEFT_SPLIT_COUNT..LEAF_NODE_MAX_CELLS + 1).rev(), old_node_ptr, new_node, key, value, value_cell_num);
            new_node.set_leaf_node_num_cells(LEAF_NODE_RIGHT_SPLIT_COUNT);
        }

        let mut is_node_root = false;
        {
            let old_node = self.get_page();
            is_node_root = old_node.is_node_root();
            copy_page_data((0..LEAF_NODE_LEFT_SPLIT_COUNT).rev(), old_node as *const Page, old_node, key, value, value_cell_num);
            old_node.set_leaf_node_num_cells(LEAF_NODE_LEFT_SPLIT_COUNT);
            old_node.set_leaf_node_next_leaf(new_page_num);
        }

        if is_node_root {
            self.create_new_node(new_page_num);
        } else {
            let old_node = self.get_page();
            let parent_page_num = old_node.get_node_parent();
            let new_max = old_node.get_node_max_key();
            let parent = self.table.pager.get_page(parent_page_num);
            parent.update_internal_node_key(old_max, new_max);
            self.table.internal_node_insert(parent_page_num, new_page_num);
        }
    }

    fn create_new_node(&mut self, right_child_page_num: usize) {
        let left_child_page_num = self.table.pager.get_unused_page_num();
        let node_max_key;
        {
            let old_node = self.get_page_view().unwrap();
            let old_node_ptr = old_node as *const Page;
            let left_child = self.table.pager.get_page(left_child_page_num);
            unsafe {
                std::ptr::copy(old_node_ptr as *const u8, left_child as *mut Page as *mut u8, PAGE_SIZE);
                left_child.set_node_root(false);
            }
            node_max_key = left_child.get_node_max_key();
        }

        let old_node = self.get_page();
        old_node.initialize_internal_node();
        old_node.set_node_root(true);
        old_node.set_internal_node_num_keys(1);
        old_node.set_internal_node_child(0, left_child_page_num);
        old_node.set_internal_node_key(0, node_max_key);
        old_node.set_internal_node_right_child(right_child_page_num);

        let root_page_num = self.table.root_page_num;
        {
            let left_child = self.table.pager.get_page(left_child_page_num);
            left_child.set_node_parent(root_page_num);
        }
        {
            let right_child = self.table.pager.get_page(right_child_page_num);
            right_child.set_node_parent(root_page_num);
        }
    }
}