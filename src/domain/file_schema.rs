use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct File {
    pub id: Uuid,
    pub file_type: FileType,
    pub number: i64,
    pub small_number: i8,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Column {
    pub id: Uuid,
    pub column_name: String,
    pub column_data_type: String,
    pub small_number: i8,
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

impl File {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Was not able to serialize.")
    }
}