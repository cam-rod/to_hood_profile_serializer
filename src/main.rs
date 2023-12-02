use std::fs::File;
use std::io::BufWriter;

use calamine::Reader;
use clap::Parser;
use serde_json::{to_writer, to_writer_pretty};

use to_hood_profile_serializer::{create_compact_collection, open_spreadsheet, parse_spreadsheet};

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    /// Specify the year of data you want to pull
    year: String,

    /// Disables pretty-printing
    #[arg(short, long)]
    raw: bool,

    /// Enable verbose output. See documentation on [`to_hood_profile_serializer::CompactEntry`] and
    /// [`to_hood_profile_serializer::VerboseEntry`] for more details.
    #[arg(short, long, help = "Enable verbose output. See documentation on CompactEntry and VerboseEntry for more details.")]
    verbose: bool,
}

fn main() {
    let opts = Cli::parse();
    let (mut path, mut workbook) = open_spreadsheet(&opts.year);

    if let Some(Ok(range)) = workbook.worksheet_range_at(0) {
        let mapped = parse_spreadsheet(range);
        path.set_extension("json");
        let outfile = File::create(path).unwrap();
        let output = BufWriter::new(outfile);

        if opts.verbose {
            if opts.raw {
                to_writer(output, &mapped).expect("Error while writing output");
            } else {
                to_writer_pretty(output, &mapped).expect("Error while writing output");
            }
        } else {
            let collection = create_compact_collection(mapped);
            if opts.raw {
                to_writer(output, &collection).expect("Error while writing output");
            } else {
                to_writer_pretty(output, &collection).expect("Error while writing output");
            }
        }
    }
}
