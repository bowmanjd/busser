use argh::FromArgs;
use busser::{read_csv_headers, csv_to_json};
use std::path::PathBuf;


/// Load CSV file into SQL Server
#[derive(Debug, FromArgs)]
struct Args {
    /// output JSON
    #[argh(option, short = 'j')]
    jsonfile: PathBuf,

    /// CSV file path
    #[argh(positional)]
    csvfile: PathBuf,
}

fn main() {
    let args: Args = argh::from_env();
    let headers = read_csv_headers(&args.csvfile).expect("headers");
    println!("{}", headers.join(", "));
    csv_to_json(&args.csvfile, &args.jsonfile).expect("json");
}
