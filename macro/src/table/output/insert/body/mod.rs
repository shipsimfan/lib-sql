use constant::InsertConstantBody;
use variable::InsertVariableBody;

mod constant;
mod variable;

mod from_input;
mod to_tokens;

/// The body of the insert function
pub enum InsertBody {
    /// The content of the SQL query for inserting can be stored in a constant
    Constant(InsertConstantBody),

    /// The content of the SQL query for inserting must be dynamic
    Variable(InsertVariableBody),
}
