use super::Output;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<'a> ToTokens for Output<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        to_tokens! { generator
            use ::data_format::Deserialize;
        }

        self.insert.to_tokens(generator);
    }
}
