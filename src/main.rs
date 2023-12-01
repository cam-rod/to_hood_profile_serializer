mod layout {
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
}

use crate::layout::{DatasetEntry, NeighbourhoodPoint};
use calamine::{open_workbook, DataType, Reader, Xlsx};
use serde_json::to_writer_pretty;
use std::fs::File;
use std::io::BufWriter;

const BASENAME: &str = "neighbourhood-profiles-2021-158-model";

fn main() {
    let path = format!("{}/{}.xlsx", env!("CARGO_MANIFEST_DIR"), BASENAME);
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Unable to open file");

    let mut mapped: Vec<DatasetEntry> = Vec::new();

    if let Some(Ok(range)) = workbook.worksheet_range_at(0) {
        let mut names: Vec<String> = Vec::new();
        for cell in range.rows().next().unwrap().iter().skip(1) {
            if let DataType::String(name) = cell {
                names.push(name.to_string())
            }
        }

        for row in range.rows().skip(1) {
            let raw_row_name = row.get(0).unwrap().as_string().unwrap();
            let row_name = raw_row_name.trim_start();
            // Create entry
            let mut row_entry = DatasetEntry::new(row_name);
            for cell in row.iter().skip(1).enumerate().map(|(i, c)| (i, c.as_i64())) {
                if let (idx, Some(value)) = cell {
                    row_entry.data.push(NeighbourhoodPoint {
                        name: names.get(idx).unwrap().to_string(),
                        value,
                    })
                }
            }

            let indent = (raw_row_name.len() - row_name.len()) / 2;
            if indent == 0 {
                mapped.push(row_entry);
            } else {
                insert_tree(mapped.last_mut().unwrap(), row_entry, indent);
            }
        }

        write_json(mapped);
    }
}

fn insert_tree(mapped: &mut DatasetEntry, row_entry: DatasetEntry, layers: usize) {
    if layers == 1 {
        mapped.nested.push(row_entry)
    } else {
        match mapped.nested.last_mut() {
            Some(last) => insert_tree(last, row_entry, layers - 1),
            None => if layers == 2 {
                insert_tree(mapped, row_entry, layers-1)
            } else {
                panic!("issue on {} with depth {}, no layers below {}", row_entry.name, layers, mapped.name)
            }
        }

    }
}

fn write_json(result: Vec<DatasetEntry>) {
    let outfile =
        File::create(format!("{}/{}.json", env!("CARGO_MANIFEST_DIR"), BASENAME)).unwrap();
    let output = BufWriter::new(outfile);

    to_writer_pretty(output, &result).expect("Error while writing output");
}
