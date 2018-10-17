#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Table {
    pub database: String,
    pub name: String,
    pub columns: Vec<Column>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ColumnType {
    Text,
    Varchar,
    Integer,
    Float,
    Double,
    Boolean,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Column {
    pub name: String,
    pub nullable: bool,
    pub datatype: ColumnType,
}
