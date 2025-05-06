mod to_tokens;

/// The body of the insert function
pub enum InsertBody {
    /// The content of the SQL query for inserting can be stored in a constant
    Constant,

    /// The content of the SQL query for inserting must be dynamic
    Variable,
}
