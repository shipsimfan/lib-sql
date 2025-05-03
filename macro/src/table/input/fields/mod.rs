use field::Field;

mod field;

mod extract;

/// The set of fields in the struct
pub struct Fields<'a> {
    /// The fields themselves
    pub fields: Vec<Field<'a>>,
}
