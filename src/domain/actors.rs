use super::super::mappers::Mapper;
use actix::prelude::*;
use r2d2;
use r2d2_postgres;

// Actors
pub struct DomainExecutor<T: Mapper> {
    // TODO: make it private using an object builder.
    pub mapper: T,
    pub postgres: r2d2::Pool<r2d2_postgres::PostgresConnectionManager>,
}

impl<T: 'static + Mapper> Actor for DomainExecutor<T> {
    type Context = SyncContext<Self>;
}
