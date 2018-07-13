use super::super::mappers::Mapper;
use super::actors::DomainExecutor;
use super::models::ModelContainer;
use actix::prelude::*;
use postgres::types::ToSql;
use r2d2;
use r2d2_postgres;
use std::collections::HashMap;
use time::PreciseTime;

// Commands aka Messages:
pub struct CommitMessage {
    pub models: Vec<ModelContainer>,
}

impl Message for CommitMessage {
    type Result = Result<String, String>;
}

trait PostgresPooledConnection {
    fn build_command(&self, table: &String, fields: &Vec<String>) -> String;
}

impl PostgresPooledConnection for r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager> {
    fn build_command(&self, table: &String, fields: &Vec<String>) -> String {
        let mut insert_args = vec![];
        let mut update_args = vec![];

        for (i, key) in fields.iter().enumerate() {
            let index = i + 1;
            insert_args.push(format!("${}", index));
            update_args.push(format!("{} = ${}", key.to_string(), index));
        }

        let x = r#""
        update {table} set
            col_int = data_table.col_int
        from
            (select
                unnest(array[5, 2]) as col_int,
                unnest(array['e05fdb8c-e4b9-4b17-8022-5f45fea7f2de'::uuid, '47e721ce-d1fc-4950-8b3f-51aac6f56ebd'::uuid]) as id
            ) as data_table
        where
            foo.id = data_table.id;
        ""#;

        format!(
            "INSERT INTO {table} ({field_names}) VALUES ", // ({insert_args});", // ON CONFLICT (id) DO UPDATE SET {update_args} WHERE {table}.id = $1",
            table = table,
            field_names = fields.join(","),
            //insert_args = insert_args.join(","),
            //update_args = update_args.join(",")
        )
    }
}

impl<T> Handler<CommitMessage> for DomainExecutor<T>
where
    T: Mapper + 'static,
{
    type Result = Result<String, String>;

    fn handle(&mut self, msg: CommitMessage, _: &mut Self::Context) -> Self::Result {
        let start = PreciseTime::now();
        let conn = self.postgres.get().unwrap();
        let mut model_map = HashMap::new();

        for mut model in msg.models.into_iter() {
            let table_name = model.table_name();

            let (field_names, stmt) = model_map
                .entry(table_name.clone())
                .or_insert_with(|| (model.fields(), vec![]));

            stmt.push(format!(
                "({})",
                field_names
                    .iter()
                    .map(|f| model.value(f))
                    .collect::<Vec<String>>()
                    .join(",")
            ));
        }

        for (model, (field_names, args)) in model_map {
            let mut query = conn.build_command(&model, &field_names);
            query.push_str(&args.join(","));
            println!("{:?}", query);
            conn.batch_execute(&query).unwrap();
        }

        let end = PreciseTime::now();
        println!("{} seconds for {} execs.", start.to(end), 3000);
        Ok(format!("Done"))
    }
}
