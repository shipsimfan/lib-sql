use super::InsertOutput;
use crate::table::input::Input;
use proc_macro_util::tokens::Identifier;

impl<'a> InsertOutput<'a> {
    /// Create the structures to generate from `input`
    pub fn from_input(input: &Input<'a>) -> Self {
        let name = Identifier::new(&format!("New{}", input.name.to_string()));

        InsertOutput {
            attributes: input.attributes.clone(),
            visibility: input.visibility.clone(),
            name,
        }
    }
}
