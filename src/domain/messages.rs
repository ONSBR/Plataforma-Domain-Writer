use super::super::infra::postgres_batch::BatchSql;
use super::actors::DomainExecutor;
use super::models::{ColumnType, MigrationCommands, ModelContainer, ModelContainerBatchSql, Table};
use actix::prelude::*;
use barrel::backend::Pg;
use barrel::connectors::DatabaseExecutor;
use barrel::*;

pub struct CommitMessage {
    pub models: Vec<ModelContainer>,
}

pub struct MigrationMessage<T: Migratable> {
    pub object: T,
    pub command: MigrationCommands,
}

pub trait Migratable {
    fn migrate(&self) {
        let mut migration = Migration::new();
        self.prepare(&mut migration);
        let mut runner = DbRunner {};
        migration.execute::<DbRunner, Pg>(&mut runner);
    }

    fn prepare(&self, migration: &mut Migration);
}

impl Migratable for Table {
    fn prepare<'a>(&self, migration: &mut Migration) {
        let cols = self.columns.clone();

        migration.create_table(self.name.clone(), move |t| {
            cols.iter().for_each(|c| {
                let col = t.add_column(
                    c.name.clone(),
                    match c.datatype {
                        ColumnType::Integer => Type::Integer,
                        _ => Type::Text,
                    },
                );

                if c.nullable {
                    col.nullable();
                }
            });
        });
    }
}

struct DbRunner {}

impl DatabaseExecutor for DbRunner {
    fn execute<S: Into<String>>(&mut self, sql: S) {
        println!("{}", sql.into());
    }
}

impl Message for CommitMessage {
    type Result = Result<String, String>;
}

impl<T: Migratable> Message for MigrationMessage<T> {
    type Result = Result<String, String>;
}

impl Handler<CommitMessage> for DomainExecutor {
    type Result = Result<String, String>;

    fn handle(&mut self, msg: CommitMessage, _: &mut Self::Context) -> Self::Result {
        for batch in BatchSql::from_containers(&msg.models) {
            if let Ok(conn) = self.postgres.get() {
                if let Err(_) = batch.execute(&conn) {
                    return Err(format!(
                        "Data could not be persisted. Error ocurred on model {}",
                        batch.table
                    ));
                }
            } else {
                return Err(String::from(
                        "Data could not be persisted due to database connection issues. Please get in touch with an administrator.",
                        ));
            }
        }

        Ok(String::from("Data successfully persisted."))
    }
}

impl<T: Migratable> Handler<MigrationMessage<T>> for DomainExecutor {
    type Result = Result<String, String>;

    fn handle(&mut self, msg: MigrationMessage<T>, _: &mut Self::Context) -> Self::Result {
        msg.object.migrate();
        Ok(String::from("Data Migrated"))
    }
}
