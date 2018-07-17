use super::models::CommandTypes;
use std::collections::HashMap;

type Builder = Box<Fn(&String, &Vec<String>, &Vec<Vec<String>>) -> String>;

fn build_insert(table: &String, fields: &Vec<String>, values: &Vec<Vec<String>>) -> String {
    format!(
        "INSERT INTO {table} ({field_names}) VALUES {values};",
        table = table,
        field_names = fields.join(","),
        values = values
            .iter()
            .map(|v| format!("({})", v.join(",")))
            .collect::<Vec<String>>()
            .join(",")
    )
}

fn build_update(table: &String, fields: &Vec<String>, values: &Vec<Vec<String>>) -> String {
    let update_args: Vec<String> = fields
        .iter()
        .filter(|f| f != &"id")
        .map(|name| format!("{name} = data_table.{name}", name = name))
        .collect();

    let mut field_values = HashMap::new();

    for value in values {
        fields.iter().enumerate().for_each(|(i, field)| {
            field_values
                .entry(field.clone())
                .or_insert_with(|| vec![])
                .push(value[i].clone());
        });
    }

    let update_arrays: Vec<String> = field_values
        .iter()
        .map(|(f, v)| {
            if f == "id" {
                format!("unnest(array[{}::uuid]) as {}", v.join(","), f)
            } else {
                format!("unnest(array[{}]) as {}", v.join(","), f)
            }
        })
        .collect();

    format!(
        "
            update {table} set {update_args}
            from (select {update_arrays}) as data_table
            where {table}.id = data_table.id;
        ",
        table = table,
        update_args = update_args.join(","),
        update_arrays = update_arrays.join(","),
    )
}

pub fn get(cmd: &CommandTypes) -> Builder {
    match cmd {
        CommandTypes::Insert => Box::new(build_insert),
        CommandTypes::Update => Box::new(build_update),
    }
}
