use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DatasetEntry {
    pub name: String,
    pub data: Vec<NeighbourhoodPoint>,
    pub nested: Vec<DatasetEntry>,
}

impl DatasetEntry {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            data: Vec::new(),
            nested: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NeighbourhoodPoint {
    pub name: String,
    pub value: i64,
}
