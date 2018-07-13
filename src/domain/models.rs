use serde_json;
use std::fmt;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelMetadata {
    pub model_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ModelState {
    Insert,
    Delete,
    Update,
}

impl fmt::Display for ModelState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
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

        if field == "id" {
            format!("'{}'", Uuid::new_v4())
        } else if val.is_string() {
            val.to_string().replace("\"", "'")
        } else {
            val.to_string()
        }
    }

    pub fn pk(&self) -> String {
        self.value(&PK_NAME.to_string()).to_string()
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
