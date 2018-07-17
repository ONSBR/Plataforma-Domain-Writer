use super::builders;
use postgres;
use r2d2;
use r2d2_postgres;
use std::fmt;

pub struct BatchSql {
    pub table: String,
    pub command: CommandTypes,
    pub fields: Vec<String>,
    pub items: Vec<Vec<String>>,
}

impl BatchSql {
    pub fn new(table: String, fields: Vec<String>, state: CommandTypes) -> Self {
        Self {
            table: table,
            fields: fields,
            items: vec![],
            command: state,
        }
    }

    pub fn execute(
        &self,
        conn: &r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>,
    ) -> Result<(), postgres::Error> {
        let command_builder = builders::get(&self.command);
        let query = command_builder(&self.table, &self.fields, &self.items);
        println!("Generated batch query for {}:\n {}", self.table, query);
        conn.batch_execute(&query)
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum CommandTypes {
    Insert,
    Update,
    //Delete,
}

impl fmt::Display for CommandTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
