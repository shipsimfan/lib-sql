use super::Output;
use proc_macro_util::{Generator, ToTokens};

impl<'a> ToTokens for Output<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.insert.to_tokens(generator);
    }
}
