use super::Output;
use proc_macro_util::{Generator, ToTokens};

impl ToTokens for Output {
    fn to_tokens(self, generator: &mut Generator) {}
}
