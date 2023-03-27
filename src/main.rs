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
    Columns(ColumnsCmd),
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

    /// SQL table name
    #[argh(option, short = 't')]
    table: String,

    /// infer SQL type
    #[argh(switch, short = 'i')]
    infer: bool,
}

/// Show CSV columns
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "columns")]
struct ColumnsCmd {
    /// CSV file path
    #[argh(positional)]
    csvfile: PathBuf,

    /// SQL table name
    #[argh(option, short = 't')]
    table: Option<String>,

    #[argh(switch, short = 'r')]
    /// get raw columns verbatim from CSV file
    raw: bool,
}

/// Get suggested SQL table schema
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "schema")]
struct SchemaCmd {
    /// CSV file path
    #[argh(positional)]
    csvfile: PathBuf,

    /// SQL table name
    #[argh(option, short = 't')]
    table: String,

    #[argh(switch, short = 'c')]
    /// use only varchars as type
    chars: bool,
}

fn columns(args: ColumnsCmd) -> Result<()> {
    let columns = busser::csv_columns(&args.csvfile, args.table, args.raw)?;
    println!("{}", columns.join(", "));
    Ok(())
}

fn schema(args: SchemaCmd) -> Result<()> {
    busser::csv_schema(&args.csvfile, &args.table)?;
    Ok(())
}

fn output(args: OutputCmd) -> Result<()> {
    if let Some(jsonfile) = args.jsonfile {
        //busser::csv_into(&args.csvfile, &jsonfile, &args.table, args.infer, true, None)?;
        busser::csv_into_json(&args.csvfile, &jsonfile, &args.table, None)?;
    }
    if let Some(bcpfile) = args.bcpfile {
        busser::csv_to_bcp(&args.csvfile, &bcpfile)?;
    }
    Ok(())
}

fn run() -> Result<()> {
    let args: Args = argh::from_env();
    match args.subcommands {
        Subcommands::Columns(args) => columns(args)?,
        Subcommands::Output(args) => output(args)?,
        Subcommands::Schema(args) => schema(args)?,
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
