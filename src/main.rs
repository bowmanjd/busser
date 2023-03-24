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
    jsonfile: Option<PathBuf>,

    /// output BCP
    #[argh(option, short = 'b')]
    bcpfile: Option<PathBuf>,

    /// infer SQL type
    #[argh(option, short = 'i')]
    sqltype: Option<String>,
}

fn run() -> Result<()> {
    let args: Args = argh::from_env();
    let headers = busser::read_csv_headers(&args.csvfile)?;
    println!("{}", headers.join(", "));
    if let Some(jsonfile) = args.jsonfile {
        busser::csv_to_json(&args.csvfile, &jsonfile)?;
    }
    if let Some(bcpfile) = args.bcpfile {
        busser::csv_to_bcp(&args.csvfile, &bcpfile)?;
    }
    if let Some(sqltype) = args.sqltype {
        if let Some(stype) = busser::infer::check_char(&sqltype) {
            println!("{}({}, {})\nfixed: {}", stype.name, stype.size, stype.scale, stype.fixed);
        }
    }
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
