use super::InsertBody;
use proc_macro_util::{Generator, ToTokens};

impl ToTokens for InsertBody {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            InsertBody::Constant(constant) => constant.to_tokens(generator),
            InsertBody::Variable(variable) => variable.to_tokens(generator),
        }
    }
}
