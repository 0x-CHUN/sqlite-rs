use crate::node::NodeType;

pub const ID_SIZE: usize = std::mem::size_of::<u32>();
pub const USERNAME_SIZE: usize = 32;
pub const EMAIL_SIZE: usize = 255;
pub const ID_OFFSET: usize = 0;
pub const USERNAME_OFFSET: usize = ID_OFFSET + ID_SIZE;
pub const EMAIL_OFFSET: usize = USERNAME_OFFSET + USERNAME_SIZE;
pub const ROW_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;
pub const PAGE_SIZE: usize = 4096;
pub const TABLE_MAX_PAGES: usize = 100;
pub const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;
pub const TABLE_MAX_ROWS: usize = TABLE_MAX_PAGES * ROWS_PER_PAGE;

/// Common Node Header Layout:
/// NODE TYPE|IS ROOT|PARENT POINTER
pub const NODE_TYPE_SIZE: usize = std::mem::size_of::<NodeType>();
pub const NODE_TYPE_OFFSET: usize = 0;
pub const IS_ROOT_SIZE: usize = std::mem::size_of::<bool>();
pub const IS_ROOT_OFFSET: usize = NODE_TYPE_SIZE;
pub const PARENT_POINTER_SIZE: usize = std::mem::size_of::<usize>();
pub const PARENT_POINTER_OFFSET: usize = IS_ROOT_SIZE + IS_ROOT_OFFSET;
pub const COMMON_NODE_HEADER_SIZE: usize = NODE_TYPE_SIZE + IS_ROOT_SIZE + PARENT_POINTER_SIZE;

/// Leaf Node Header Layout:
/// Common Node Header|Cell num of Leaf Node
pub const LEAF_NODE_NUM_CELLS_SIZE: usize = std::mem::size_of::<usize>();
pub const LEAF_NODE_NUM_CELLS_OFFSET: usize = COMMON_NODE_HEADER_SIZE;
pub const LEAF_NODE_NEXT_LEAF_SIZE: usize = std::mem::size_of::<usize>();
pub const LEAF_NODE_NEXT_LEAF_OFFSET: usize = LEAF_NODE_NUM_CELLS_OFFSET + LEAF_NODE_NUM_CELLS_SIZE;
pub const LEAF_NODE_HEADER_SIZE: usize = COMMON_NODE_HEADER_SIZE + LEAF_NODE_NUM_CELLS_SIZE + LEAF_NODE_NEXT_LEAF_SIZE;

/// Leaf Node Body Layout:
/// [Leaf Node Key|Leaf Node Value]
pub const LEAF_NODE_KEY_SIZE: usize = std::mem::size_of::<u32>();
pub const LEAF_NODE_KEY_OFFSET: usize = 0;
pub const LEAF_NODE_VALUE_SIZE: usize = ROW_SIZE;
pub const LEAF_NODE_VALUE_OFFSET: usize = LEAF_NODE_KEY_OFFSET + LEAF_NODE_KEY_SIZE;
pub const LEAF_NODE_CELL_SIZE: usize = LEAF_NODE_KEY_SIZE + LEAF_NODE_VALUE_SIZE;
pub const LEAF_NODE_SPACE_FOR_CELLS: usize = PAGE_SIZE - LEAF_NODE_HEADER_SIZE;
pub const LEAF_NODE_MAX_CELLS: usize = LEAF_NODE_SPACE_FOR_CELLS / LEAF_NODE_CELL_SIZE;
pub const LEAF_NODE_RIGHT_SPLIT_COUNT: usize = (LEAF_NODE_MAX_CELLS + 1) / 2;
pub const LEAF_NODE_LEFT_SPLIT_COUNT: usize = (LEAF_NODE_MAX_CELLS + 1) - LEAF_NODE_RIGHT_SPLIT_COUNT;

/// Internal Node Header Layout
pub const INTERNAL_NODE_NUM_KEYS_SIZE: usize = std::mem::size_of::<usize>();
pub const INTERNAL_NODE_NUM_KEYS_OFFSET: usize = COMMON_NODE_HEADER_SIZE;
pub const INTERNAL_NODE_RIGHT_CHILD_SIZE: usize = std::mem::size_of::<usize>();
pub const INTERNAL_NODE_RIGHT_CHILD_OFFSET: usize = INTERNAL_NODE_NUM_KEYS_OFFSET + INTERNAL_NODE_NUM_KEYS_SIZE;
pub const INTERNAL_NODE_HEADER_SIZE: usize = COMMON_NODE_HEADER_SIZE + INTERNAL_NODE_NUM_KEYS_SIZE + INTERNAL_NODE_RIGHT_CHILD_SIZE;

/// Internal Node Body Layout
pub const INTERNAL_NODE_KEY_SIZE: usize = std::mem::size_of::<u32>();
pub const INTERNAL_NODE_CHILD_SIZE: usize = std::mem::size_of::<usize>();
pub const INTERNAL_NODE_CELL_SIZE: usize = INTERNAL_NODE_KEY_SIZE + INTERNAL_NODE_CHILD_SIZE;

pub const INTERNAL_NODE_MAX_CELLS: usize = 100;


pub fn print_constants() {
    println!("ROW_SIZE: {}", ROW_SIZE);
    println!("COMMON_NODE_HEADER_SIZE: {}", COMMON_NODE_HEADER_SIZE);
    println!();
    println!("LEAF_NODE_HEADER_SIZE: {}", LEAF_NODE_HEADER_SIZE);
    println!("LEAF_NODE_CELL_SIZE: {}", LEAF_NODE_CELL_SIZE);
    println!("LEAF_NODE_SPACE_FOR_CELLS: {}", LEAF_NODE_SPACE_FOR_CELLS);
    println!("LEAF_NODE_MAX_CELLS: {}", LEAF_NODE_MAX_CELLS);
    println!();
    println!("INTERNAL_NODE_HEADER_SIZE: {}", INTERNAL_NODE_HEADER_SIZE);
    println!("INTERNAL_NODE_KEY_SIZE: {}", INTERNAL_NODE_KEY_SIZE);
    println!("INTERNAL_NODE_CHILD_SIZE: {}", INTERNAL_NODE_CHILD_SIZE);
    println!("INTERNAL_NODE_CELL_SIZE: {}", INTERNAL_NODE_CELL_SIZE);
}