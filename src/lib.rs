use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use calamine::{open_workbook, DataType, Range, Xlsx};

use layout::DatasetEntry;

use crate::layout::NeighbourhoodPoint;

pub mod layout;

pub const BASENAME: &str = "neighbourhood-profiles-";

pub fn open_spreadsheet(year: String) -> (PathBuf, Xlsx<BufReader<File>>) {
    let path = PathBuf::from(format!("{}{}.xlsx", BASENAME, year));
    let workbook: Xlsx<BufReader<File>> = open_workbook(&path)
        .unwrap_or_else(|_| panic!("Unable to open spreadsheet at {}.", path.to_str().unwrap()));
    (path, workbook)
}

pub fn parse_spreadsheet(range: Range<DataType>) -> Vec<DatasetEntry> {
    let mut mapped: Vec<DatasetEntry> = Vec::new();

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

    mapped
}

pub fn insert_tree(mapped: &mut DatasetEntry, row_entry: DatasetEntry, layers: usize) {
    if layers == 1 {
        mapped.nested.push(row_entry)
    } else {
        match mapped.nested.last_mut() {
            Some(last) => insert_tree(last, row_entry, layers - 1),
            None => {
                if layers == 2 {
                    insert_tree(mapped, row_entry, layers - 1)
                } else {
                    panic!(
                        "issue on {} with depth {}, no layers below {}",
                        row_entry.name, layers, mapped.name
                    )
                }
            }
        }
    }
}
