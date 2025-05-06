use proc_macro_util::{tokens::Literal, Generator, ToTokens};

/// Generates places that can accept variables to be bound in an SQL query
pub struct BindSites {
    /// The number of binds to produce
    amount: usize,
}

impl BindSites {
    /// Create a new set of [`Binds`]
    pub fn new(amount: usize) -> Self {
        BindSites { amount }
    }
}

impl ToTokens for BindSites {
    fn to_tokens(self, generator: &mut Generator) {
        Literal::new(self.to_string().as_str()).to_tokens(generator);
    }
}

impl std::fmt::Display for BindSites {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.amount {
            if i > 0 {
                write!(f, ", ")?;
            }

            write!(f, "?")?;
        }

        Ok(())
    }
}
