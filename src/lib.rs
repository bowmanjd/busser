use anyhow::{Context, Result};
use csv::{Reader, ReaderBuilder, Terminator};
use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::iter::zip;
use std::path::{Path, PathBuf};

pub mod infer;
mod keywords;

type HeaderGen = fn(&mut BufWriter<File>, &str, &[String]) -> Result<()>;
type FooterGen = fn(&mut BufWriter<File>, &str, &[String], &Vec<infer::SQLType>) -> Result<()>;
type FieldProcessor = fn(&mut BufWriter<File>, &str, &str) -> Result<()>;

struct OutputConfig {
    row_sep: String,
    field_sep: String,
    field_processor: FieldProcessor,
    page_header: Option<HeaderGen>,
    page_footer: Option<FooterGen>,
}

fn csv_reader(
    csvfile: &PathBuf,
    field_sep: Option<u8>,
    row_sep: Option<u8>,
) -> Result<Reader<File>> {
    let field_sep = field_sep.unwrap_or(b',');
    let sep: Terminator;
    if let Some(row_sep) = row_sep {
        sep = Terminator::Any(row_sep);
    } else {
        sep = Terminator::CRLF;
    }
    let rdr = ReaderBuilder::new()
        .delimiter(field_sep)
        .terminator(sep)
        .from_path(csvfile)
        .with_context(|| format!("Failed to read csv from {:?}", csvfile))?;
    Ok(rdr)
}

// TODO: de-duplicate column names when needed
pub fn csv_columns(
    csvfile: &PathBuf,
    tablename: Option<&str>,
    raw: bool,
    field_sep: Option<u8>,
    row_sep: Option<u8>,
) -> Result<Vec<String>> {
    let mut rdr = csv_reader(csvfile, field_sep, row_sep)?;
    let headers = rdr.headers()?;
    let new_headers: Vec<String> = if raw {
        headers.iter().map(str::to_string).collect()
    } else {
        headers
            .iter()
            .map(|h| {
                let clean_chars: String = h
                    .chars()
                    .map(|c| if c.is_alphanumeric() { c } else { '_' })
                    .collect();

                if keywords::KEYWORDS.contains(&&clean_chars.to_ascii_lowercase()[..]) {
                    let prefix: char;
                    if let Some(tablename) = &tablename {
                        prefix = tablename.chars().next().unwrap_or('x');
                    } else {
                        prefix = csvfile
                            .to_str()
                            .unwrap_or("x")
                            .chars()
                            .next()
                            .unwrap_or('x');
                    }
                    format!("{}_{}", prefix, clean_chars)
                } else {
                    clean_chars
                }
            })
            .collect()
    };
    Ok(new_headers)
}

pub fn csv_schema(csvfile: &PathBuf, tablename: &str, ascii_delimited: bool) -> Result<String> {
    let field_sep: Option<u8>;
    let row_sep: Option<u8>;
    if ascii_delimited {
        field_sep = Some(b'\x1F');
        row_sep = Some(b'\x1E');
    } else {
        field_sep = None;
        row_sep = None;
    }
    let headers = csv_columns(csvfile, Some(tablename), false, field_sep, row_sep)?;
    let mut rdr = csv_reader(csvfile, field_sep, row_sep)?;
    let row_length: usize = headers.len();
    let mut sqltypes: Vec<infer::SQLType> = vec![
        infer::SQLType {
            name: "bit".to_string(),
            ..Default::default()
        };
        row_length
    ];

    for result in rdr.records() {
        let row = result?;
        for (i, value) in row.iter().enumerate() {
            if let Some(sqltype) = infer::infer(value, sqltypes[i].index, sqltypes[i].subindex) {
                sqltypes[i].merge(&sqltype);
            }
        }
    }
    let schema = schema_string(&headers, &sqltypes);
    Ok(format!(
        "DROP TABLE IF EXISTS {0};\nCREATE TABLE {0} ({1});",
        tablename, schema
    ))
}

fn field_processor_bcp(stream: &mut BufWriter<File>, _column: &str, value: &str) -> Result<()> {
    write!(stream, "{}", value)?;
    Ok(())
}

fn field_processor_json(stream: &mut BufWriter<File>, column: &str, value: &str) -> Result<()> {
    let mut new_value = String::new();
    if value.is_empty() {
        new_value.push_str("null");
    } else {
        new_value.push('"');
        for char in value.chars() {
            match char {
                '\\' => new_value.push_str("\\\\"),
                '"' => new_value.push_str("\\\""),
                '\'' => new_value.push_str("''"),
                _ => new_value.push(char),
            }
        }
        new_value.push('"');
    }
    write!(stream, "\"{}\": {}", column, &new_value)?;
    Ok(())
}

fn page_header_json(
    stream: &mut BufWriter<File>,
    tablename: &str,
    columns: &[String],
) -> Result<()> {
    write!(stream, "INSERT INTO {}\nSELECT\n", tablename)?;
    for (i, col) in columns.iter().enumerate() {
        if i > 0 {
            writeln!(stream, ",")?;
        }
        write!(stream, "    {}", col)?;
    }
    write!(stream, "\nFROM OPENJSON('[ \\\n    {{")?;
    Ok(())
}

fn page_footer_json(
    stream: &mut BufWriter<File>,
    _tablename: &str,
    columns: &[String],
    sqltypes: &Vec<infer::SQLType>,
) -> Result<()> {
    let schema = schema_string(columns, sqltypes);
    writeln!(stream, "}} \\\n]') WITH ({});", schema)?;
    Ok(())
}

pub fn csv_into_bcp(
    csvfile: &PathBuf,
    filename: &PathBuf,
    tablename: &str,
    infer: bool,
    page_size: usize,
) -> Result<()> {
    let conf = OutputConfig {
        row_sep: String::from('\x1E'),
        field_sep: String::from('\x1F'),
        field_processor: field_processor_bcp,
        page_header: None,
        page_footer: None,
    };
    csv_into(csvfile, filename, tablename, infer, page_size, conf)
}

pub fn csv_into_json(
    csvfile: &PathBuf,
    filename: &PathBuf,
    tablename: &str,
    page_size: usize,
) -> Result<()> {
    let conf = OutputConfig {
        row_sep: String::from("}, \\\n    {"),
        field_sep: String::from(", "),
        field_processor: field_processor_json,
        page_header: Some(page_header_json),
        page_footer: Some(page_footer_json),
    };
    csv_into(csvfile, filename, tablename, true, page_size, conf)
}

fn indexed_file_path<T>(path: T, index: usize) -> PathBuf
where
    T: AsRef<Path>,
{
    let path = path.as_ref();
    let mut newpath = path.to_owned();
    if index > 0 {
        let mut stem = OsString::new();

        if let Some(s) = path.file_stem() {
            stem.push(s);
        } else {
            stem.push("output");
        }

        stem.push("_");
        stem.push(index.to_string());

        newpath.set_file_name(stem);

        if let Some(ext) = path.extension() {
            newpath.set_extension(ext);
        }
    }
    newpath
}

fn new_file(outpath: impl AsRef<Path>, index: usize) -> Result<BufWriter<File>> {
    let outfile = File::create(indexed_file_path(outpath, index))?;
    Ok(BufWriter::new(outfile))
}

pub fn determine_output_path(
    path: Option<impl AsRef<Path>>,
    tablename: &str,
    extension: &str,
) -> Result<PathBuf> {
    let mut outfile = PathBuf::new();
    if let Some(path) = path {
        let path = path.as_ref();
        outfile.push(path);
    } else {
        outfile.push("out");
    }
    if outfile.extension().is_none() && !outfile.exists() {
        fs::create_dir_all(outfile.clone())?;
    }
    if outfile.is_dir() {
        outfile.set_file_name(tablename);
        outfile.set_extension(extension);
    }
    Ok(outfile)
}

fn csv_into(
    csvfile: &PathBuf,
    outpath: &PathBuf,
    tablename: &str,
    infer: bool,
    page_size: usize,
    config: OutputConfig,
) -> Result<()> {
    let mut page: usize = 0;
    let columns = csv_columns(csvfile, Some(tablename), false, None, None)?;
    let mut rdr = csv_reader(csvfile, None, None)?;
    let mut stream = new_file(outpath, page)?;
    let mut sqltypes: Vec<infer::SQLType> = vec![
        infer::SQLType {
            name: "bit".to_string(),
            ..Default::default()
        };
        columns.len()
    ];
    let mut new_page = true;
    if page_size > 0 {
        page = 1;
    }

    for (rounds, result) in rdr.records().enumerate() {
        if page_size > 0 && rounds > 0 && (rounds % page_size) == 0 {
            if let Some(page_footer) = config.page_footer {
                page_footer(&mut stream, tablename, &columns, &sqltypes)?;
            }
            stream.flush()?;
            page += 1;
            stream = new_file(outpath, page)?;
            new_page = true;
        }
        if new_page {
            new_page = false;
            if let Some(page_header) = config.page_header {
                page_header(&mut stream, tablename, &columns)?;
            }
        } else {
            write!(stream, "{}", config.row_sep)?;
        }
        let row = result?;
        for (i, (column, value)) in zip(&columns, &row).enumerate() {
            if infer {
                if let Some(sqltype) = infer::infer(value, sqltypes[i].index, sqltypes[i].subindex)
                {
                    sqltypes[i].merge(&sqltype);
                }
            }
            if i != 0 {
                write!(stream, "{}", config.field_sep)?;
            }
            (config.field_processor)(&mut stream, column, value)?;
        }
    }
    if let Some(page_footer) = config.page_footer {
        page_footer(&mut stream, tablename, &columns, &sqltypes)?;
    }
    stream.flush()?;
    let schema = schema_string(&columns, &sqltypes);
    println!(
        "DROP TABLE IF EXISTS {0};\nCREATE TABLE {0} ({1});",
        tablename, schema
    );
    Ok(())
}

fn schema_string(columns: &[String], sqltypes: &Vec<infer::SQLType>) -> String {
    let mut schema = String::new();
    for (i, (column, sqlt)) in zip(columns, sqltypes).enumerate() {
        if i > 0 {
            schema.push_str(", ");
        }
        schema.push_str(&format!("{} {}", column, sqlt));
    }
    schema
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nonexistent_csv_file() {
        let attempt = csv_reader(&PathBuf::from("No_Such_File.csv"), None, None);
        assert!(attempt.is_err());
    }
}
