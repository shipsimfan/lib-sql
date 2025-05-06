use crate::{Bind, Statement};

impl Bind for [u8] {
    fn bind<'statement, S: Statement<'statement>>(
        &'statement self,
        idx: usize,
        statement: &mut S,
    ) -> Result<(), S::Error> {
        statement.bind_blob(idx, self)
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

impl Bind for Vec<u8> {
    fn bind<'statement, S: Statement<'statement>>(
        &'statement self,
        idx: usize,
        statement: &mut S,
    ) -> Result<(), S::Error> {
        statement.bind_blob(idx, self)
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

impl Bind for Box<[u8]> {
    fn bind<'statement, S: Statement<'statement>>(
        &'statement self,
        idx: usize,
        statement: &mut S,
    ) -> Result<(), S::Error> {
        statement.bind_blob(idx, self)
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
