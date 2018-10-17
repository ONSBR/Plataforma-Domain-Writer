use barrel::backend::Pg;
use barrel::connectors::DatabaseExecutor;
use barrel::*;
use domain::models::schema::{ColumnType, Table};
use r2d2::PooledConnection;
use r2d2_postgres::PostgresConnectionManager;

pub struct PostgresDatabaseMigrator {
    pub connection: PooledConnection<PostgresConnectionManager>,
}

impl DatabaseExecutor for PostgresDatabaseMigrator {
    fn execute<S: Into<String>>(&mut self, sql: S) {
        let commands = sql.into();
        println!("{}", commands);
        match self.connection.execute(&commands, &[]) {
            Ok(x) => println!("Ok::{}", x),
            Err(u) => println!("{}", u),
        }
    }
}

pub trait Migratable {
    fn migrate<T: DatabaseExecutor>(&self, mut executor: T) {
        let mut migration = Migration::new().schema("xpto");
        self.prepare(&mut migration);
        migration.execute::<T, Pg>(&mut executor);
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

#[derive(Serialize, Deserialize, Debug)]
pub enum MigrationCommands {
    Create,
    Update,
    Delete,
}
