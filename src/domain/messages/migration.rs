use actix::prelude::*;
use domain::actors::DomainExecutor;
use domain::models::migrations::{PostgresDatabaseMigrator, Migratable, MigrationCommands};

pub struct MigrationMessage<T: Migratable> {
    pub object: T,
    pub command: MigrationCommands,
}

impl<T: Migratable> Message for MigrationMessage<T> {
    type Result = Result<String, String>;
}

impl<T: Migratable> Handler<MigrationMessage<T>> for DomainExecutor {
    type Result = Result<String, String>;

    fn handle(&mut self, msg: MigrationMessage<T>, _: &mut Self::Context) -> Self::Result {
        match self.postgres.get() {
            Ok(connection) => {
                let migrator = PostgresDatabaseMigrator { connection };
                msg.object.migrate(migrator);
                Ok("done".to_string())
            }
            Err(_) => Err(String::from("err")),
        }
    }
}
pub struct Messagge {



}

