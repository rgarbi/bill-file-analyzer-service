use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct FileDefinition {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub file_type: FileType,
    pub columns: Vec<Column>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Column {
    pub id: Uuid,
    pub column_name: String,
    pub column_data_type: ColumnDataType,
    pub column_description: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum ColumnDataType {
    STRING,
    NUMBER,
    CURRENCY,
    DATE,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum FileType {
    JSON,
    CSV,
}

impl FileType {
    pub fn as_str(&self) -> &'static str {
        match self {
            FileType::JSON => "JSON",
            FileType::CSV => "CSV",
        }
    }
}

impl FileDefinition {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Was not able to serialize.")
    }
}
