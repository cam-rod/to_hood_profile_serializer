use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

use calamine::Reader;
use clap::Parser;
use serde_json::{to_writer, to_writer_pretty};

use to_hood_profile_serializer::layout::DatasetEntry;
use to_hood_profile_serializer::{open_spreadsheet, parse_spreadsheet};

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    /// Specify the year of data you want to pull
    year: String,

    /// Disables pretty-printing
    #[arg(short, long)]
    raw: bool,
}

fn main() {
    let opts = Cli::parse();
    let (path, mut workbook) = open_spreadsheet(opts.year);

    if let Some(Ok(range)) = workbook.worksheet_range_at(0) {
        let mapped = parse_spreadsheet(range);
        write_json(path, mapped, opts.raw);
    }
}

fn write_json(mut path: PathBuf, result: Vec<DatasetEntry>, raw: bool) {
    path.set_extension("json");
    let outfile = File::create(path).unwrap();
    let output = BufWriter::new(outfile);

    if raw {
        to_writer(output, &result).expect("Error while writing output");
    } else {
        to_writer_pretty(output, &result).expect("Error while writing output");
    }
}
