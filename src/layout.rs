use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type NeighbourhoodPoints = HashMap<String, i64>;
pub type CompactCollection = HashMap<String, CompactEntry>;

/// Compact representation of spreadsheet data.
///
/// ```json
/// {
///   "Population": {
///     "data": {
///       "North York": 300000,
///       "Scarborough": 370000
///     },
///     "nested": {
///       "Pop under 12": {
///         "data": {
///           "North York": 120000,
///           "Scarborough": 140000
///         },
///         "nested": {}
///       }
///     }
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CompactEntry {
    pub data: NeighbourhoodPoints,
    pub nested: CompactCollection,
}

/// Verbose representation of spreadsheet data.
///
/// ```json
/// {
///   "name": "Population",
///   "data": {
///     "North York": 300000,
///     "Scarborough": 370000
///   },
///   "nested": [
///     {
///       "name": "Pop under 12",
///       "data": {
///         "North York": 120000,
///         "Scarborough": 140000
///       },
///       "nested": []
///     }
///   ]
/// }
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct VerboseEntry {
    /// This field is only used for proper nesting
    pub name: String,
    pub data: NeighbourhoodPoints,
    pub nested: Vec<Self>,
}

impl VerboseEntry {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            data: NeighbourhoodPoints::default(),
            nested: Vec::default(),
        }
    }

    pub fn get_name(self) -> String {
        self.name
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NeighbourhoodPoint {
    pub name: String,
    pub value: i64,
}

#[cfg(test)]
mod tests {
    use super::{CompactCollection, CompactEntry, NeighbourhoodPoints, VerboseEntry};

    #[test]
    fn test_serialize_compact() {
        let dataset = CompactCollection::from([(
            "Population".to_string(),
            CompactEntry {
                data: NeighbourhoodPoints::from_iter(vec![
                    ("North York".to_string(), 300000),
                    ("Scarborough".to_string(), 370000),
                ]),
                nested: CompactCollection::from([(
                    "Pop under 12".to_string(),
                    CompactEntry {
                        data: NeighbourhoodPoints::from_iter(vec![
                            ("Scarborough".to_string(), 140000),
                            ("North York".to_string(), 120000),
                        ]),
                        nested: CompactCollection::default(),
                    },
                )]),
            },
        )]);
        println!("{}", serde_json::to_string_pretty(&dataset).unwrap());
    }

    #[test]
    fn test_serialize_verbose() {
        let dataset = VerboseEntry {
            name: "Population".to_string(),
            data: NeighbourhoodPoints::from_iter(vec![
                ("North York".to_string(), 300000),
                ("Scarborough".to_string(), 370000),
            ]),
            nested: vec![VerboseEntry {
                name: "Pop under 12".to_string(),
                data: NeighbourhoodPoints::from_iter(vec![
                    ("Scarborough".to_string(), 140000),
                    ("North York".to_string(), 120000),
                ]),
                nested: vec![],
            }],
        };
        println!("{}", serde_json::to_string_pretty(&dataset).unwrap());
    }
}
