use super::super::mappers::Mapper;
use super::actors::DomainExecutor;
use super::models::{ModelContainer, ModelState};
use actix::prelude::*;
//use postgres::types::ToSql;
//use r2d2;
//use r2d2_postgres;
use std::collections::HashMap;
use time::PreciseTime;

// Commands aka Messages:
pub struct CommitMessage {
    pub models: Vec<ModelContainer>,
}

impl Message for CommitMessage {
    type Result = Result<String, String>;
}

//trait PostgresPooledConnection {
//    fn build_command(&self, command: Command, table: &String, fields: &Vec<String>) -> String;
//}

//impl CommandBuilder for UpdateCommandBuilder {
//    fn build(table: &String, fields: &Vec<String>) -> String {
//        let x = r#""
//        update {table} set
//            col_int = data_table.col_int
//        from
//            (select
//                unnest(array[5, 2]) as col_int,
//                unnest(array['e05fdb8c-e4b9-4b17-8022-5f45fea7f2de'::uuid, '47e721ce-d1fc-4950-8b3f-51aac6f56ebd'::uuid]) as id
//            ) as data_table
//        where
//            foo.id = data_table.id;
//        ""#;

//        format!(
//            "INSERT INTO {table} ({field_names}) VALUES ", // ({insert_args});", // ON CONFLICT (id) DO UPDATE SET {update_args} WHERE {table}.id = $1",
//            table = table,
//            field_names = fields.join(","),
//        )
//    }
//}

//impl CommandBuilder for InsertCommandBuilder {
//}
mod command_builders {
    use domain::models::ModelState;
    use std::collections::HashMap;

    type Builder = Box<Fn(&String, &Vec<String>, &Vec<Vec<String>>) -> String>;

    fn build_insert(table: &String, fields: &Vec<String>, values: &Vec<Vec<String>>) -> String {
        let insert_values = values
            .iter()
            .map(|v| format!("({})", v.join(",")))
            .collect::<Vec<String>>()
            .join(",");

        format!(
            "INSERT INTO {table} ({field_names}) VALUES {values};",
            table = table,
            field_names = fields.join(","),
            values = insert_values
        )
    }

    fn build_update(table: &String, fields: &Vec<String>, values: &Vec<Vec<String>>) -> String {
        let update_args: Vec<String> = fields
            .iter()
            .filter(|f| f != &"id")
            .map(|name| format!("{name} = data_table.{name}", name = name))
            .collect();

        let mut mp = HashMap::new();

        for value in values {
            for (i, field) in fields.iter().enumerate() {
                let m = mp.entry(field.clone()).or_insert_with(|| vec![]);
                m.push(value[i].clone());
            }
        }

        let update_arrays: Vec<String> = mp.iter()
            .map(|(f, v)| {
                if f == "id" {
                    return format!("unnest(array[{}::uuid]) as {}", v.join(","), f);
                }
                format!("unnest(array[{}]) as {}", v.join(","), f)
            })
            .collect();

        format!(
            "
        update {table} set
            {update_args}
        from
            (select
                {update_arrays}
            ) as data_table
        where
            foo.id = data_table.id;
        ",
            table = table,
            update_args = update_args.join(",\n\t"),
            update_arrays = update_arrays.join(",\n\t\t"),
        )
    }

    pub fn get(cmd: &ModelState) -> Builder {
        match cmd {
            ModelState::Insert => Box::new(build_insert),
            ModelState::Update => Box::new(build_update),
        }
    }
}

//use self::command_builders;

//impl PostgresPooledConnection for r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager> {
//    fn build_command(&self, command: Command, table: &String, fields: &Vec<String>) -> String {
//        command_builders::
//            //let builder = CommandBuilders::get(command);
//            //builder.build(table, fields)
//            String::new()
//    }
//}
//

struct ModelMap {
    model: String,
    fields: Vec<String>,
    items: HashMap<ModelState, Vec<Vec<String>>>,
}

impl ModelMap {
    fn from_container(container: &ModelContainer) -> Self {
        let mut items = HashMap::new();
        items.insert(ModelState::Insert, vec![]);
        items.insert(ModelState::Update, vec![]);

        Self {
            model: container.table_name().clone(),
            fields: container.fields(),
            items: items,
        }
    }

    fn apppend_item(&mut self, container: ModelContainer) {
        self.items.get_mut(&container.metadata.state).unwrap().push(
            self.fields
                .iter()
                .map(|f| container.value(f))
                .collect::<Vec<String>>(),
        );
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
        let mut model_maps = HashMap::new();

        for mut model in msg.models.into_iter() {
            let map = model_maps
                .entry(model.table_name().clone())
                .or_insert_with(|| ModelMap::from_container(&model));
            map.apppend_item(model);
        }

        for map in model_maps.values() {
            map.items.iter().for_each(|(state, values)| {
                if !values.is_empty() {
                    let command_builder = command_builders::get(state);
                    let query = command_builder(&map.model, &map.fields, values);
                    println!("{}", query);
                    conn.batch_execute(&query).unwrap();
                }
            });
        }

        let end = PreciseTime::now();
        println!("{} seconds for {} execs.", start.to(end), 3000);
        Ok(format!("Done"))
    }
}
