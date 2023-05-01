use anyhow::Result;
use argh::FromArgs;
use std::path::PathBuf;

/// Prepare tables for SQL Server
#[derive(Debug, FromArgs)]
pub struct Args {
    #[argh(subcommand)]
    subcommands: Subcommands,
}

#[derive(Debug, PartialEq, FromArgs)]
#[argh(subcommand)]
enum Subcommands {
    Output(OutputCmd),
    Columns(ColumnsCmd),
    Schema(SchemaCmd),
    View(ViewCmd),
}

/// Output special formats from CSV input
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "output")]
struct OutputCmd {
    /// CSV file path
    #[argh(positional)]
    csvfile: PathBuf,

    /// output file or directory
    #[argh(option, short = 'o')]
    output: Option<PathBuf>,

    /// output JSON
    #[argh(switch, short = 'j')]
    json: bool,

    /// SQL table name
    #[argh(option, short = 't')]
    table: String,

    /// infer SQL type
    #[argh(switch, short = 'i')]
    infer: bool,

    /// rows per page (0 for no paging)
    #[argh(option, short = 'p', default = "0")]
    pagesize: usize,
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

/// View CSV file
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "view")]
struct ViewCmd {
    /// CSV file path
    #[argh(positional)]
    csvfile: PathBuf,

    #[argh(switch, short = 'a')]
    /// use only varchars as type
    asciidelimited: bool,

    #[argh(switch, short = 'n')]
    /// use only varchars as type
    numbered: bool,

    #[argh(option, short = 'r')]
    /// show only row or range of rows
    rows: Option<String>,

    #[argh(option, short = 'c')]
    /// show only row or range of rows
    columns: Option<String>,
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

    #[argh(switch, short = 'a')]
    /// use only varchars as type
    asciidelimited: bool,

    #[argh(switch, short = 'c')]
    /// use only varchars as type
    chars: bool,
}

fn view(args: ViewCmd) -> Result<()> {
    /*
    let stats = busser::csv_survey(
        &args.csvfile,
        false,
        None,
        None,
        None,
        )?;
    println!("{:?}", stats);
    */
    busser::view::view(
        &args.csvfile,
        args.rows.as_deref(),
        args.columns.as_deref(),
        None,
        None,
        args.asciidelimited,
        args.numbered,
    )?;
    Ok(())
}

fn columns(args: ColumnsCmd) -> Result<()> {
    let columns = busser::csv_columns(&args.csvfile, args.table.as_deref(), args.raw, None, None)?;
    println!("{}", columns.join(", "));
    Ok(())
}

fn schema(args: SchemaCmd) -> Result<()> {
    let create_table = busser::csv_schema(&args.csvfile, &args.table, args.asciidelimited)?;
    println!("{}", create_table);
    Ok(())
}

fn output(args: OutputCmd) -> Result<()> {
    let extension: String = if args.json {
        "sql".to_string()
    } else {
        "txt".to_string()
    };
    let outfile = busser::determine_output_path(args.output, &args.table, &extension)?;
    if args.json {
        busser::csv_into_json(&args.csvfile, &outfile, &args.table, args.pagesize)?;
    } else {
        busser::csv_into_bcp_fast(
            &args.csvfile,
            &outfile,
            &args.table,
            args.infer,
            args.pagesize,
        )?;
    }
    Ok(())
}

pub fn run(args: Args) -> Result<()> {
    match args.subcommands {
        Subcommands::Columns(args) => columns(args)?,
        Subcommands::Output(args) => output(args)?,
        Subcommands::Schema(args) => schema(args)?,
        Subcommands::View(args) => view(args)?,
    }

    Ok(())
}
