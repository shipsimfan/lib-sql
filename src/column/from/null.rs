use crate::{Column, FromColumn};
use std::marker::PhantomData;

impl FromColumn for () {
    fn from_column<'a, C: Column<'a>>(_: C) -> Result<Self, C::Error> {
        Ok(())
    }
}

impl<T> FromColumn for PhantomData<T> {
    fn from_column<'a, C: Column<'a>>(_: C) -> Result<Self, C::Error> {
        Ok(PhantomData)
    }
}
