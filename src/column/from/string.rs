use crate::{Column, FromColumn};

impl FromColumn for String {
    fn from_column<'column, C: Column<'column>>(column: C) -> Result<Self, C::Error> {
        column.into_str().map(|str| str.to_string())
    }

    fn validate<E: crate::FromColumnError>(
        &self,
        min: Option<f64>,
        max: Option<f64>,
    ) -> Result<(), E> {
        let min = min.unwrap_or(1.0) as usize;
        if self.len() < min {
            return Err(E::custom("value is too short"));
        }

        if let Some(max) = max {
            if self.len() > max as _ {
                return Err(E::custom("value is too long"));
            }
        }

        Ok(())
    }
}
