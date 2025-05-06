use proc_macro_util::{to_tokens, Generator, ToTokens};

pub struct DeriveDeserialize;

impl ToTokens for DeriveDeserialize {
    fn to_tokens(self, generator: &mut Generator) {
        to_tokens! { generator
            #[derive(Deserialize)]
        }
    }
}
