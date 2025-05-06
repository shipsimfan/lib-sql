use super::InsertStructField;
use proc_macro_util::{to_tokens, Generator, ToTokens, Token};

impl<'a> ToTokens for InsertStructField<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let InsertStructField {
            attributes,
            name,
            r#type,
            has_default,
        } = self;

        to_tokens! { generator
            #attributes
            pub #name:
        }

        if has_default {
            to_tokens! { generator
                ::core::option::Option<
            }
        }

        r#type.to_tokens(generator);

        if has_default {
            Token![>]().to_tokens(generator);
        }

        Token![,]().to_tokens(generator);
    }
}
