use crate::{Column, FromColumn};

impl FromColumn for u8 {
    fn from_column<'a, C: Column<'a>>(column: C) -> Result<u8, C::Error> {
        column.into_u8()
    }
}

impl FromColumn for u16 {
    fn from_column<'a, C: Column<'a>>(column: C) -> Result<u16, C::Error> {
        column.into_u16()
    }
}

impl FromColumn for u32 {
    fn from_column<'a, C: Column<'a>>(column: C) -> Result<u32, C::Error> {
        column.into_u32()
    }
}

impl FromColumn for u64 {
    fn from_column<'a, C: Column<'a>>(column: C) -> Result<u64, C::Error> {
        column.into_u64()
    }
}

impl FromColumn for usize {
    fn from_column<'a, C: Column<'a>>(column: C) -> Result<usize, C::Error> {
        column.into_usize()
    }
}

impl FromColumn for i8 {
    fn from_column<'a, C: Column<'a>>(column: C) -> Result<i8, C::Error> {
        column.into_i8()
    }
}

impl FromColumn for i16 {
    fn from_column<'a, C: Column<'a>>(column: C) -> Result<i16, C::Error> {
        column.into_i16()
    }
}

impl FromColumn for i32 {
    fn from_column<'a, C: Column<'a>>(column: C) -> Result<i32, C::Error> {
        column.into_i32()
    }
}

impl FromColumn for i64 {
    fn from_column<'a, C: Column<'a>>(column: C) -> Result<i64, C::Error> {
        column.into_i64()
    }
}

impl FromColumn for isize {
    fn from_column<'a, C: Column<'a>>(column: C) -> Result<isize, C::Error> {
        column.into_isize()
    }
}

impl FromColumn for f32 {
    fn from_column<'a, C: Column<'a>>(column: C) -> Result<f32, C::Error> {
        column.into_f32()
    }
}

impl FromColumn for f64 {
    fn from_column<'a, C: Column<'a>>(column: C) -> Result<f64, C::Error> {
        column.into_f64()
    }
}
