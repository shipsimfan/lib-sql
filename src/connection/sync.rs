use crate::Connection;
use std::sync::{MutexGuard, RwLockWriteGuard};

impl<'a, T: Connection> Connection for MutexGuard<'a, T> {
    type Statement<'statement>
        = T::Statement<'statement>
    where
        'a: 'statement;
    type Transaction<'transaction>
        = T::Transaction<'transaction>
    where
        'a: 'transaction;
    type ExecuteError = T::ExecuteError;
    type PrepareError = T::PrepareError;

    fn execute(&mut self, sql: &str) -> Result<(), Self::ExecuteError> {
        T::execute(&mut *self, sql)
    }

    fn prepare<'statement>(
        &'statement mut self,
        sql: &str,
    ) -> Result<Self::Statement<'statement>, Self::PrepareError> {
        T::prepare(&mut *self, sql)
    }

    fn begin_trasaction<'transaction>(
        &'transaction mut self,
    ) -> Result<Self::Transaction<'transaction>, Self::ExecuteError> {
        T::begin_trasaction(&mut *self)
    }
}

impl<'a, T: Connection> Connection for RwLockWriteGuard<'a, T> {
    type Statement<'statement>
        = T::Statement<'statement>
    where
        'a: 'statement;
    type Transaction<'transaction>
        = T::Transaction<'transaction>
    where
        'a: 'transaction;
    type ExecuteError = T::ExecuteError;
    type PrepareError = T::PrepareError;

    fn execute(&mut self, sql: &str) -> Result<(), Self::ExecuteError> {
        T::execute(&mut *self, sql)
    }

    fn prepare<'statement>(
        &'statement mut self,
        sql: &str,
    ) -> Result<Self::Statement<'statement>, Self::PrepareError> {
        T::prepare(&mut *self, sql)
    }

    fn begin_trasaction<'transaction>(
        &'transaction mut self,
    ) -> Result<Self::Transaction<'transaction>, Self::ExecuteError> {
        T::begin_trasaction(&mut *self)
    }
}
