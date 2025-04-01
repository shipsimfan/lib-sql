use std::sync::MutexGuard;

use crate::{SQLite3Connection, SQLite3ConnectionPool};
use sql::ConnectionPool;

impl ConnectionPool for SQLite3ConnectionPool {
    type Connection = SQLite3Connection;

    type Guard<'connection> = MutexGuard<'connection, Self::Connection>;

    fn get_connection<'connection>(&'connection self) -> Self::Guard<'connection> {
        self.connection.lock().unwrap()
    }
}
