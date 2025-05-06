mod field;

mod extract;

pub use field::Field;

/// The set of fields in the struct
pub struct Fields<'a> {
    /// The fields themselves
    pub fields: Vec<Field<'a>>,
}
