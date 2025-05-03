use super::{insert::InsertOutput, Output};
use crate::table::{Input, TableFlags};

impl<'a> Output<'a> {
    /// Create the structures to generate from `input`
    pub fn from_input(input: Input<'a>, flags: TableFlags) -> Self {
        let insert = if !flags.not_insertable {
            Some(InsertOutput::from_input(&input))
        } else {
            None
        };

        Output { insert }
    }
}
