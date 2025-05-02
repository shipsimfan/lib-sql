use crate::Connection;
use std::ops::{Deref, DerefMut};

/// A pool of connections to a database
pub trait ConnectionPool: 'static + Send + Sync {
    /// The connection type of the database
    type Connection<'connection>: Connection<'connection>;

    /// The returned guard type representing access to a connection in the pool
    type Guard<'connection>: 'connection + Deref<Target = Self::Connection<'connection>> + DerefMut;

    /// Get a connection from the pool, blocking if one isn't available right away
    fn get_connection<'connection>(&'connection self) -> Self::Guard<'connection>;
}
