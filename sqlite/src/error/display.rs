use crate::SQLite3Error;

impl std::fmt::Display for SQLite3Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SQLite3Error::Database(error) => error.fmt(f),
            SQLite3Error::MissingColumn(name) => write!(f, "missing column \"{}\"", name),
            SQLite3Error::InvalidValue(column, error) => {
                write!(f, "invalid value for column \"{}\" - {}", column, error)
            }
            SQLite3Error::Custom(message) => message.fmt(f),
        }
    }
}
