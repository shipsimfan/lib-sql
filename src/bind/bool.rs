use crate::Bind;

impl Bind for bool {
    fn bind<'statement, S: crate::Statement<'statement>>(
        &'statement self,
        idx: usize,
        statement: &mut S,
    ) -> Result<(), S::Error> {
        statement.bind_u8(idx, if *self { 1 } else { 0 })
    }
}
