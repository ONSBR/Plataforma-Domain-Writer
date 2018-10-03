use infra::postgres_batch::{BatchSql, CommandTypes};
use serde_json;
use std::collections::HashMap;
use std::fmt;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelMetadata {
    pub model_type: String,
    pub state: CommandTypes,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelContainer {
    pub metadata: ModelMetadata,
    pub entity: serde_json::Value,
}

impl fmt::Display for ModelContainer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

const PK_NAME: &'static str = "id";

impl ModelContainer {
    pub fn table_name(&self) -> &String {
        &self.metadata.model_type
    }

    pub fn value(&self, field: &String) -> String {
        let val = self.entity.get(field).unwrap();

        if field == PK_NAME {
            format!("'{}'", Uuid::new_v4())
        } else if val.is_string() {
            val.to_string().replace("\"", "'")
        } else {
            val.to_string()
        }
    }

    pub fn fields(&self) -> Vec<String> {
        let mut fields: Vec<String> = vec![PK_NAME.to_string()];

        if let Some(entity) = self.entity.as_object() {
            entity
                .iter()
                .filter(|(k, _)| *k != PK_NAME)
                .for_each(|(k, _)| fields.push(k.clone()));
        }

        fields
    }
}

pub trait ModelContainerBatchSql {
    fn apppend_item(&mut self, container: &ModelContainer);
    fn from_containers(containers: &Vec<ModelContainer>) -> Vec<BatchSql>;
}

impl ModelContainerBatchSql for BatchSql {
    fn apppend_item(&mut self, container: &ModelContainer) {
        self.items.push(
            self.fields
                .iter()
                .map(|f| container.value(f))
                .collect::<Vec<String>>(),
        );
    }

    fn from_containers(containers: &Vec<ModelContainer>) -> Vec<BatchSql> {
        let mut batches = HashMap::new();

        for mut model in containers.into_iter() {
            let batch = batches
                .entry((model.table_name().clone(), model.metadata.state.clone()))
                .or_insert_with(|| {
                    BatchSql::new(
                        model.table_name().clone(),
                        model.fields(),
                        model.metadata.state,
                    )
                });

            batch.apppend_item(model);
        }

        batches.into_iter().map(|(_, v)| v).collect()
    }
}


