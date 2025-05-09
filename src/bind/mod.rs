use crate::{FromColumnError, Statement};

mod blob;
mod cow;
mod null;
mod numeric;
mod option;
mod string;

/// A data structure which can be bound to as an SQL statement parameter
pub trait Bind {
    /// Bind this structure to `statement` at `idx`
    fn bind<'statement, S: Statement<'statement>>(
        &'statement self,
        idx: usize,
        statement: &mut S,
    ) -> Result<(), S::Error>;

    /// Validate the either the length or value of this element
    #[allow(unused_variables)]
    fn validate<E: FromColumnError>(&self, min: Option<f64>, max: Option<f64>) -> Result<(), E> {
        Ok(())
    }
}
