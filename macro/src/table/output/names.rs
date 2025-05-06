use proc_macro_util::{tokens::Literal, Generator, ToTokens};

/// Generates a series of names that can be places into an SQL query
pub struct Names {
    /// The names to produce
    names: Vec<String>,
}

impl Names {
    /// Create a new set of [`Names`]
    pub fn new(names: Vec<String>) -> Self {
        Names { names }
    }
}

impl ToTokens for Names {
    fn to_tokens(self, generator: &mut Generator) {
        Literal::new(self.to_string().as_str()).to_tokens(generator);
    }
}

impl std::fmt::Display for Names {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.names.len() {
            if i > 0 {
                write!(f, ", ")?;
            }

            write!(f, "\"{}\"", self.names[i])?;
        }

        Ok(())
    }
}
