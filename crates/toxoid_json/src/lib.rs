use serde::{Serialize, Deserialize};
pub use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonEntity {
    pub path: String,
    pub label: String,
    pub ids: Vec<Vec<String>>,
    pub values: Vec<serde_json::Value>,
    pub refs: Option<serde_json::Value>,
}

pub fn parse_entity(entity: &str) -> JsonEntity {
    let entity: JsonEntity = serde_json::from_str(entity).unwrap();
    entity
}