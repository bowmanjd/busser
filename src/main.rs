use anyhow::Result;
use argh::FromArgs;
use std::{path::PathBuf, process};

/// Prepare tables for SQL Server
#[derive(Debug, FromArgs)]
struct Args {
    #[argh(subcommand)]
    subcommands: Subcommands,
}

#[derive(Debug, PartialEq, FromArgs)]
#[argh(subcommand)]
enum Subcommands {
    Output(OutputCmd),
    Header(HeaderCmd),
    Schema(SchemaCmd),
}

/// Output special formats from CSV input
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "output")]
struct OutputCmd {
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
    #[argh(switch, short = 'i')]
    infer: bool,
}

/// Show CSV header
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "header")]
struct HeaderCmd {
    /// CSV file path
    #[argh(positional)]
    csvfile: PathBuf,

    #[argh(switch, short = 'r')]
    /// get raw headers verbatim from CSV file
    raw: bool,
}

/// Get suggested SQL table schema
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "schema")]
struct SchemaCmd {
    #[argh(switch, short = 'c')]
    /// use only varchars as type
    chars: bool,
}

fn header(args: HeaderCmd) -> Result<()> {
    let headers = busser::read_csv_headers(&args.csvfile)?;
    println!("{}", headers.join(", "));
    Ok(())
}

fn output(args: OutputCmd) -> Result<()> {
    if let Some(jsonfile) = args.jsonfile {
        busser::csv_to_json(&args.csvfile, &jsonfile, args.infer)?;
    }
    if let Some(bcpfile) = args.bcpfile {
        busser::csv_to_bcp(&args.csvfile, &bcpfile)?;
    }
    Ok(())
}

fn run() -> Result<()> {
    let args: Args = argh::from_env();
    match args.subcommands {
        Subcommands::Header(args) => header(args)?,
        Subcommands::Output(args) => output(args)?,
        Subcommands::Schema(args) => {
            println!("args: {:?}", args);
        },
    }

    /*
    let headers = busser::read_csv_headers(&args.csvfile)?;
    println!("{}", headers.join(", "));
    if let Some(jsonfile) = args.jsonfile {
        busser::csv_to_json(&args.csvfile, &jsonfile)?;
    }
    if let Some(bcpfile) = args.bcpfile {
        busser::csv_to_bcp(&args.csvfile, &bcpfile)?;
    }
    if let Some(sqltype) = args.sqltype {
        if let Some(stype) = busser::infer::infer(&sqltype, 0) {
            println!(
                "{}({}, {})\nfixed: {}\nindex: {}",
                stype.name, stype.size, stype.scale, stype.fixed, stype.index
            );
        }
    }
    */
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
