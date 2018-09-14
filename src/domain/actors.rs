use actix::prelude::*;
use r2d2;
use r2d2_postgres;

// Actors
pub struct DomainExecutor {
    // TODO: make it private using an object builder.
    pub postgres: r2d2::Pool<r2d2_postgres::PostgresConnectionManager>,
}

impl Actor for DomainExecutor {
    type Context = SyncContext<Self>;
}
