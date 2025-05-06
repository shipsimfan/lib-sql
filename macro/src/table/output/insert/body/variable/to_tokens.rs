use super::InsertVariableBody;
use proc_macro_util::{Generator, ToTokens};

impl ToTokens for InsertVariableBody {
    fn to_tokens(self, generator: &mut Generator) {}
}
