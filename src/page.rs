use std::io::Read;
use std::process;
use crate::node::NodeType;
use crate::node::NodeType::{NodeInternal, NodeLeaf};
use crate::row::Row;
use crate::constant::*;

pub struct Page {
    pub(crate) buf: [u8; PAGE_SIZE],
}

impl Page {
    pub(crate) fn new() -> Self {
        Page {
            buf: [0; PAGE_SIZE]
        }
    }

    pub(crate) unsafe fn row_mut_slot(&self, cell_num: usize) -> Box<Row> {
        fn read_end_idx(bytes: &[u8]) -> usize {
            for i in (0..bytes.len()).rev() {
                if bytes[i] != 0 {
                    return i;
                }
            }
            0
        }
        let cell = self.leaf_node_value(cell_num);

        let id = std::ptr::read(cell as *const u32);
        let username_bytes = std::ptr::read((cell as usize + USERNAME_OFFSET) as *const [u8; USERNAME_SIZE]);
        let email_bytes = std::ptr::read((cell as usize + EMAIL_OFFSET) as *const [u8; EMAIL_SIZE]);

        Box::new(Row {
            id,
            username: String::from_utf8_unchecked(Vec::from(&username_bytes[0..=read_end_idx(&username_bytes)])),
            email: String::from_utf8_unchecked(Vec::from(&email_bytes[0..=read_end_idx(&email_bytes)])),
        })
    }

    fn load(&mut self, bytes: &[u8]) {
        let mut idx = 0;
        while idx + ROW_SIZE <= bytes.len() {
            let mut reader = std::io::Cursor::new(&bytes[idx..idx + ROW_SIZE]);
            let mut id_bytes = [0; ID_SIZE];
            reader.read_exact(&mut id_bytes)
                .expect("Page load error");
            let mut username_bytes = [0; USERNAME_SIZE];
            reader.read_exact(&mut username_bytes)
                .expect("Page load error");
            let mut email_bytes = [0; EMAIL_SIZE];
            reader.read_exact(&mut email_bytes)
                .expect("Page load error");
            idx += ROW_SIZE;
        }
    }

    unsafe fn leaf_node_mut_num_cells(&self) -> *mut usize {
        self.index(LEAF_NODE_NUM_CELLS_OFFSET) as *mut usize
    }

    pub(crate) fn leaf_node_num_cells(&self) -> usize {
        unsafe { *self.leaf_node_mut_num_cells() }
    }

    pub(crate) fn set_leaf_node_num_cells(&mut self, num_cells: usize) {
        unsafe {
            *self.leaf_node_mut_num_cells() = num_cells
        }
    }

    fn index(&self, offset: usize) -> isize {
        let ptr = self.buf.as_ptr();
        (ptr as isize).checked_add(offset as isize).unwrap()
    }

    pub(crate) fn leaf_node_cell(&self, cell_num: usize) -> *const u8 {
        (self.index(LEAF_NODE_HEADER_SIZE + cell_num * LEAF_NODE_CELL_SIZE)) as *const u8
    }

    pub(crate) fn leaf_node_key(&self, cell_num: usize) -> u32 {
        unsafe { *(self.leaf_node_cell(cell_num) as *mut u32) }
    }

    pub(crate) fn set_leaf_node_key(&self, cell_num: usize, key: u32) {
        unsafe { *(self.leaf_node_cell(cell_num) as *mut u32) = key }
    }

    pub(crate) fn leaf_node_value(&self, cell_num: usize) -> *mut u8 {
        self.index(LEAF_NODE_HEADER_SIZE + cell_num * LEAF_NODE_CELL_SIZE + LEAF_NODE_KEY_SIZE) as *mut u8
    }

    pub(crate) fn initialize_leaf_node(&mut self) {
        self.set_node_type(NodeLeaf);
        self.set_node_root(false);
        self.set_leaf_node_next_leaf(0);
        let ptr = self.index(LEAF_NODE_NUM_CELLS_OFFSET) as *mut usize;
        unsafe {
            *ptr = 0;
        }
    }

    pub(crate) fn initialize_internal_node(&mut self) {
        self.set_node_type(NodeInternal);
        self.set_node_root(false);
        let ptr = self.index(INTERNAL_NODE_NUM_KEYS_OFFSET) as *mut usize;
        unsafe {
            *ptr = 0;
        }
    }

    fn is_full(&self) -> bool {
        self.leaf_node_num_cells() >= LEAF_NODE_MAX_CELLS
    }

    pub(crate) fn is_leaf_node(&self) -> bool {
        *(self.get_node_type()) == NodeType::NodeLeaf
    }

    pub(crate) fn get_node_type<'a>(&self) -> &'a NodeType {
        unsafe { &*(self.index(NODE_TYPE_OFFSET) as *const NodeType) }
    }

    fn set_node_type(&mut self, node_type: NodeType) {
        let ptr = self.index(NODE_TYPE_OFFSET) as *mut u8;
        unsafe {
            *ptr = node_type as u8;
        }
    }

    pub fn is_node_root(&self) -> bool {
        unsafe { *(self.index(IS_ROOT_OFFSET) as *const bool) }
    }

    pub fn set_node_root(&mut self, is_root: bool) {
        unsafe {
            *(self.index(IS_ROOT_OFFSET) as *mut bool) = is_root;
        }
    }

    fn internal_node_right_child(&self) -> isize {
        self.index(INTERNAL_NODE_RIGHT_CHILD_OFFSET)
    }

    pub fn set_internal_node_right_child(&mut self, internal_node_right_child: usize) {
        unsafe {
            *(self.internal_node_right_child() as *mut usize) = internal_node_right_child;
        }
    }

    pub fn get_internal_node_right_child(&self) -> usize {
        unsafe {
            *(self.internal_node_right_child() as *mut usize)
        }
    }

    pub fn set_internal_node_num_keys(&mut self, num_keys: usize) {
        unsafe {
            *(self.index(INTERNAL_NODE_NUM_KEYS_OFFSET) as *mut usize) = num_keys;
        }
    }

    pub fn get_internal_node_num_keys(&self) -> usize {
        unsafe {
            *(self.index(INTERNAL_NODE_NUM_KEYS_OFFSET) as *const usize)
        }
    }

    pub fn increase_internal_node_num_keys(&mut self, incr: usize) {
        let origin_num_keys = self.get_internal_node_num_keys();
        self.set_internal_node_num_keys(origin_num_keys + incr);
    }

    pub fn internal_node_cell(&self, cell_num: usize) -> isize {
        self.index(INTERNAL_NODE_HEADER_SIZE + cell_num * INTERNAL_NODE_CELL_SIZE)
    }

    fn set_internal_node_cell(&mut self, cell_num: usize, page_num: usize) {
        unsafe { *(self.internal_node_cell(cell_num) as *mut usize) = page_num }
    }

    fn get_internal_node_cell(&self, cell_num: usize) -> usize {
        unsafe { *(self.internal_node_cell(cell_num) as *const usize) }
    }

    pub fn set_internal_node_child(&mut self, child_num: usize, child_page_num: usize) {
        let num_keys = self.get_internal_node_num_keys();
        if child_num > num_keys {
            println!("Tried to access child_num {} > num_keys {}", child_num, num_keys);
            process::exit(0x0010);
        } else if child_num == num_keys {
            self.set_internal_node_right_child(child_page_num);
        } else {
            self.set_internal_node_cell(child_num, child_page_num);
        }
    }

    pub fn get_internal_node_child(&self, child_num: usize) -> usize {
        let num_keys = self.get_internal_node_num_keys();
        if child_num > num_keys {
            println!("Tried to access child_num {}", child_num);
            process::exit(0x0010);
        } else if child_num == num_keys {
            self.get_internal_node_right_child()
        } else {
            self.get_internal_node_cell(child_num)
        }
    }

    pub fn set_internal_node_key(&mut self, key_num: usize, key_val: u32) {
        unsafe {
            *((self.internal_node_cell(key_num) + INTERNAL_NODE_CHILD_SIZE as isize) as *mut u32) = key_val;
        }
    }

    pub(crate) fn get_internal_node_key(&self, cell_num: usize) -> u32 {
        unsafe {
            *((self.internal_node_cell(cell_num) + INTERNAL_NODE_CHILD_SIZE as isize) as *const u32)
        }
    }

    pub fn get_node_max_key(&self) -> u32 {
        match self.get_node_type() {
            NodeInternal => self.get_internal_node_key(self.get_internal_node_num_keys() - 1),
            NodeLeaf => self.leaf_node_key(self.leaf_node_num_cells() - 1)
        }
    }

    pub fn get_leaf_node_next_leaf(&self) -> usize {
        unsafe {
            *(self.index(LEAF_NODE_NEXT_LEAF_OFFSET) as *const usize)
        }
    }

    pub fn set_leaf_node_next_leaf(&self, next_leaf: usize) {
        unsafe {
            *(self.index(LEAF_NODE_NEXT_LEAF_OFFSET) as *mut usize) = next_leaf;
        }
    }

    pub fn get_node_parent(&self) -> usize {
        unsafe {
            *(self.index(PARENT_POINTER_OFFSET) as *const usize)
        }
    }

    pub fn set_node_parent(&self, parent_page_num: usize) {
        unsafe {
            *(self.index(PARENT_POINTER_OFFSET) as *mut usize) = parent_page_num;
        }
    }

    pub fn update_internal_node_key(&mut self, old_key: u32, new_key: u32) {
        let old_child_index = self.internal_node_find_child(old_key);
        self.set_internal_node_key(old_child_index, new_key);
    }

    /// Return the index of the child which should contain the given key.
    pub(crate) fn internal_node_find_child(&self, key: u32) -> usize {
        let num_keys = self.get_internal_node_num_keys();
        // binary search
        let (mut min_cell, mut max_cell) = (0, num_keys);
        while min_cell < max_cell {
            let cell_num = (max_cell - min_cell) / 2 + min_cell;
            let cell_key_value = self.get_internal_node_key(cell_num);
            if cell_key_value >= key {
                max_cell = cell_num;
            } else {
                min_cell = cell_num + 1;
            }
        }
        max_cell
    }

    pub(crate) fn leaf_node_find(&self, key: u32) -> usize {
        let num_cells = self.leaf_node_num_cells();
        let (mut min_index, mut one_past_max_index) = (0, num_cells);
        while one_past_max_index != min_index {
            let index = (one_past_max_index + min_index) / 2;
            let key_at_index = self.leaf_node_key(index);
            if key_at_index == key {
                return index;
            } else if key_at_index > key {
                one_past_max_index = index;
            } else {
                min_index = index + 1;
            }
        }
        min_index
    }
}