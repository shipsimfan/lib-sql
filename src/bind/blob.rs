use crate::{Bind, Statement};

impl Bind for [u8] {
    fn bind<'statement, S: Statement<'statement>>(
        &'statement self,
        idx: usize,
        statement: &mut S,
    ) -> Result<(), S::BindError> {
        statement.bind_blob(idx, self)
    }
}

impl Bind for Vec<u8> {
    fn bind<'statement, S: Statement<'statement>>(
        &'statement self,
        idx: usize,
        statement: &mut S,
    ) -> Result<(), S::BindError> {
        statement.bind_blob(idx, self)
    }
}

impl Bind for Box<[u8]> {
    fn bind<'statement, S: Statement<'statement>>(
        &'statement self,
        idx: usize,
        statement: &mut S,
    ) -> Result<(), S::BindError> {
        statement.bind_blob(idx, self)
    }
}
