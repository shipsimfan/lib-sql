use crate::SQLite3Error;

impl std::fmt::Display for SQLite3Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SQLite3Error::Execute(error) => error.fmt(f),
            SQLite3Error::FromRow(error) => error.fmt(f),
            SQLite3Error::Prepare(error) => error.fmt(f),
        }
    }
}
