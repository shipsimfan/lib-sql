use flags::TableFlags;
use input::Input;
use output::Output;
use proc_macro_util::{ast::Item, Result};

mod flags;
mod input;
mod output;

/// Convert `item` and `flags` into a set of structs representing a table
pub fn table(item: Item, flags: TableFlags) -> Result<Output> {
    Input::extract(item).map(|input| Output::from_input(input, flags))
}
