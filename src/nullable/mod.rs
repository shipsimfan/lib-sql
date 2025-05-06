mod bind;
mod from;
mod from_column;
mod into;

#[cfg(feature = "lib-data-format")]
mod deserialize;
#[cfg(feature = "lib-data-format")]
mod serialize;

/// An element which can be null
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Nullable<T> {
    /// The element is not null
    NonNull(T),

    /// The element is null
    Null,
}
