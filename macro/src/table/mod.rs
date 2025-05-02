use flags::TableFlags;
use proc_macro_util::{ast::Item, Result};

mod flags;

/// Convert `item` and `flags` into a set of structs representing a table
pub fn table(item: Item, flags: TableFlags) -> Result<Item> {
    todo!();
}
