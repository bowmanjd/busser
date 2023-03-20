use anyhow::Result;
use argh::FromArgs;
use std::{path::PathBuf, process};

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

fn run() -> Result<()> {
    let args: Args = argh::from_env();
    let headers = busser::read_csv_headers(&args.csvfile)?;
    println!("{}", headers.join(", "));
    busser::csv_to_json(&args.csvfile, &args.jsonfile)?;
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
