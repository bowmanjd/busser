use argh::FromArgs;
use std::path::PathBuf;

/// Load CSV file into SQL Server
#[derive(Debug, FromArgs)]
struct Args {
    /// CSV file path
    #[argh(positional)]
    csvfile: PathBuf,

    /// output JSON
    #[argh(option, short = 'j')]
    jsonfile: PathBuf,
}

fn main() {
    let args: Args = argh::from_env();
    let headers = busser::read_csv_headers(&args.csvfile).expect("headers");
    println!("{}", headers.join(", "));
    busser::csv_to_json(&args.csvfile, &args.jsonfile).expect("json");
}
