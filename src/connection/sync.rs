use crate::{Connection, SqlContext};
use std::sync::{MutexGuard, RwLockWriteGuard};

impl<'connection, T: Connection<'connection>> Connection<'connection>
    for MutexGuard<'connection, T>
{
    type Transaction<'transaction>
        = T::Transaction<'transaction>
    where
        'connection: 'transaction;

    fn begin_trasaction<'transaction>(
        &'transaction mut self,
    ) -> Result<Self::Transaction<'transaction>, Self::Error>
    where
        'connection: 'transaction,
    {
        T::begin_trasaction(&mut *self)
    }
}

impl<'context, T: SqlContext<'context>> SqlContext<'context> for MutexGuard<'context, T> {
    type Statement<'statement>
        = T::Statement<'statement>
    where
        'context: 'statement;
    type Error = T::Error;

    fn execute(&mut self, sql: &str) -> Result<(), Self::Error> {
        T::execute(&mut *self, sql)
    }

    fn prepare<'statement>(
        &'statement mut self,
        sql: &str,
    ) -> Result<Self::Statement<'statement>, Self::Error>
    where
        'context: 'statement,
    {
        T::prepare(&mut *self, sql)
    }
}

impl<'connection, T: Connection<'connection>> Connection<'connection>
    for RwLockWriteGuard<'connection, T>
{
    type Transaction<'transaction>
        = T::Transaction<'transaction>
    where
        'connection: 'transaction;

    fn begin_trasaction<'transaction>(
        &'transaction mut self,
    ) -> Result<Self::Transaction<'transaction>, Self::Error>
    where
        'connection: 'transaction,
    {
        T::begin_trasaction(&mut *self)
    }
}

impl<'context, T: SqlContext<'context>> SqlContext<'context> for RwLockWriteGuard<'context, T> {
    type Statement<'statement>
        = T::Statement<'statement>
    where
        'context: 'statement;
    type Error = T::Error;

    fn execute(&mut self, sql: &str) -> Result<(), Self::Error> {
        T::execute(&mut *self, sql)
    }

    fn prepare<'statement>(
        &'statement mut self,
        sql: &str,
    ) -> Result<Self::Statement<'statement>, Self::Error>
    where
        'context: 'statement,
    {
        T::prepare(&mut *self, sql)
    }
}
