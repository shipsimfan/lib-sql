use proc_macro_util::{to_tokens, tokens::Identifier, Generator, ToTokens};

/// Binds a set of fields in `self` to a variable named `statement`
pub struct Binds {
    /// The index to use for the first bind
    starting_index: usize,

    /// The names of the variable to bind
    names: Vec<Identifier>,
}

impl Binds {
    /// Creates a new [`Binds`]
    pub fn new(starting_index: usize, names: Vec<Identifier>) -> Self {
        Binds {
            starting_index,
            names,
        }
    }
}

impl ToTokens for Binds {
    fn to_tokens(self, generator: &mut Generator) {
        let Binds {
            starting_index,
            names,
        } = self;

        let mut index = starting_index;
        for name in names {
            to_tokens! { generator
                statement.bind(#index, &self.#name)?;
            }

            index += 1;
        }
    }
}
