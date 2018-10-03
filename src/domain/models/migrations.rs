use domain::models::schema::{ColumnType, Table};
use domain::messages::migration::DbRunner;
use barrel::*;
use barrel::backend::Pg;

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

#[derive(Serialize, Deserialize, Debug)]
pub enum MigrationCommands {
    Create,
    Update,
    Delete,
}
