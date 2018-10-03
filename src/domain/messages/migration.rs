use domain::actors::DomainExecutor;
use domain::models::migrations::{Migratable, MigrationCommands};
use actix::prelude::*;
use barrel::connectors::DatabaseExecutor;

pub struct MigrationMessage<T: Migratable> {
    pub object: T,
    pub command: MigrationCommands,
}

pub struct DbRunner {}

impl DatabaseExecutor for DbRunner {
    fn execute<S: Into<String>>(&mut self, sql: S) {
        println!("{}", sql.into());
    }
}

impl<T: Migratable> Message for MigrationMessage<T> {
    type Result = Result<String, String>;
}


impl<T: Migratable> Handler<MigrationMessage<T>> for DomainExecutor {
    type Result = Result<String, String>;

    fn handle(&mut self, msg: MigrationMessage<T>, _: &mut Self::Context) -> Self::Result {
        msg.object.migrate();
        Ok(String::from("Data Migrated"))
    }
}
