use anyhow::{Context, Result};
use csv::{Reader, ReaderBuilder};
use serde_json::{Map, Value};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::iter::zip;
use std::path::PathBuf;

pub mod infer;
mod keywords;

fn csv_reader(csvfile: &PathBuf) -> Result<Reader<File>> {
    let rdr = ReaderBuilder::new()
        .from_path(csvfile)
        .with_context(|| format!("Failed to read csv from {:?}", csvfile))?;
    Ok(rdr)
}

// TODO: de-duplicate column names when needed
pub fn csv_columns(csvfile: &PathBuf, tablename: Option<&str>, raw: bool) -> Result<Vec<String>> {
    let new_headers: Vec<String>;
    let mut rdr = ReaderBuilder::new()
        .from_path(csvfile)
        .with_context(|| format!("Failed to read csv from {:?}", csvfile))?;
    let headers = rdr.headers()?;
    if raw {
        new_headers = headers.iter().map(str::to_string).collect();
    } else {
        new_headers = headers
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
            .collect();
    }
    Ok(new_headers)
}

pub fn csv_schema(csvfile: &PathBuf, tablename: &str) -> Result<String> {
    let headers = csv_columns(csvfile, Some(tablename), false)?;
    let mut rdr = csv_reader(csvfile)?;
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

pub fn csv_into_json(
    csvfile: &PathBuf,
    filename: &PathBuf,
    tablename: &str,
    page_size: Option<usize>,
) -> Result<()> {
    let headers = csv_columns(csvfile, Some(tablename), false)?;
    let row_sep = ", \\\n";
    let mut json_row: Map<String, Value> = Map::new();
    let page_size: usize = page_size.unwrap_or(5000);
    let mut rdr = csv_reader(csvfile)?;
    let outfile = File::create(filename)?;
    let mut stream = BufWriter::new(outfile);
    let mut first_line = true;
    let row_length: usize = headers.len();
    let mut sqltypes: Vec<infer::SQLType> = vec![
        infer::SQLType {
            ..Default::default()
        };
        row_length
    ];

    for (row_num, result) in rdr.records().enumerate() {
        if !first_line {
            write!(stream, "{}", row_sep)?;
        } else {
            writeln!(
                stream,
                "INSERT INTO {}\nSELECT * FROM OPENJSON('[ \\",
                tablename
            )?;
            first_line = false;
        }
        let row = result?;
        json_row.clear();
        //for i in 0..row_length {
        for (i, (column, value)) in zip(&headers, &row).enumerate() {
            //let column = &headers[i];
            //let value = &row[i];
            if let Some(sqltype) = infer::infer(value, sqltypes[i].index, sqltypes[i].subindex) {
                sqltypes[i].merge(&sqltype);
            }
            json_row.insert(column.into(), value.into());
        }
        let json_text = serde_json::to_string(&json_row)?.replace("'", "''");
        //let fields = row.iter().collect::<Vec<&str>>();

        write!(stream, "{}", json_text)?;
        if ((row_num + 1) % (page_size)) == 0 {
            writeln!(
                stream,
                " \\\n]') WITH ({});\n",
                schema_string(&headers, &sqltypes)
            )?;
            first_line = true;
        }
    }
    let schema = schema_string(&headers, &sqltypes);
    if !first_line {
        writeln!(stream, " \\\n]') WITH ({});", schema)?;
    }

    stream.flush()?;
    println!(
        "DROP TABLE IF EXISTS {0};\nCREATE TABLE {0} ({1});",
        tablename, schema
    );
    Ok(())
}

fn field_processor_json(stream: &mut BufWriter<File>, column: &str, value: &str) -> Result<()> {
    let mut new_value = String::new();
    if value.is_empty() {
        new_value.push_str("null");
    } else {
        new_value.push('"');
        new_value.push_str(&value
            .replace("\\", "\\\\")
            .replace("\"", "\\\"")
            .replace("'", "''"));
        new_value.push('"');
    }
    write!(
        stream,
        "\"{}\": {}",
        column,
        new_value
    )?;
    Ok(())
}

fn page_header_json(
    stream: &mut BufWriter<File>,
    tablename: &str,
    columns: &Vec<String>,
) -> Result<()> {
    write!(stream, "INSERT INTO {}\nSELECT\n", tablename)?;
    for (i, col) in columns.iter().enumerate() {
        if i > 0 {
            write!(stream, ",\n")?;
        }
        write!(stream, "    {}", col)?;
    }
    write!(stream, "\nFROM OPENJSON('[ \\\n    {{")?;
    Ok(())
}

fn page_footer_json(
    stream: &mut BufWriter<File>,
    _tablename: &str,
    columns: &Vec<String>,
    sqltypes: &Vec<infer::SQLType>,
) -> Result<()> {
    let schema = schema_string(columns, sqltypes);
    writeln!(stream, "}} \\\n]') WITH ({});", schema)?;
    Ok(())
}

pub fn csv_into_json2(
    csvfile: &PathBuf,
    filename: &PathBuf,
    tablename: &str,
    page_size: usize,
) -> Result<()> {
    csv_into(
        csvfile,
        filename,
        tablename,
        true,
        "}, \\\n    {",
        ", ",
        &(field_processor_json as fn(&mut BufWriter<File>, &str, &str) -> Result<()>),
        page_size,
        Some(&(page_header_json as fn(&mut BufWriter<File>, &str, &Vec<String>) -> Result<()>)),
        Some(&(page_footer_json as fn(&mut BufWriter<File>, &str, &Vec<String>, &Vec<infer::SQLType>) -> Result<()>)),
    )
}
pub fn csv_into(
    csvfile: &PathBuf,
    filename: &PathBuf,
    tablename: &str,
    infer: bool,
    row_sep: &str,
    field_sep: &str,
    field_processor: &fn(&mut BufWriter<File>, &str, &str) -> Result<()>,
    page_size: usize,
    page_header: Option<&fn(&mut BufWriter<File>, &str, &Vec<String>) -> Result<()>>,
    page_footer: Option<&fn(&mut BufWriter<File>, &str, &Vec<String>, &Vec<infer::SQLType>) -> Result<()>>,
) -> Result<()> {
    let columns = csv_columns(csvfile, Some(tablename), false)?;
    let mut rdr = csv_reader(csvfile)?;
    let outfile = File::create(filename)?;
    let mut stream = BufWriter::new(outfile);
    let row_length: usize = columns.len();
    let mut sqltypes: Vec<infer::SQLType> = vec![
        infer::SQLType {
            name: "bit".to_string(),
            ..Default::default()
        };
        row_length
    ];

    for (rounds, result) in rdr.records().enumerate() {
        if rounds > 0 {
            write!(stream, "{}", row_sep)?;
        } else if let Some(page_header) = page_header {
            page_header(&mut stream, tablename, &columns)?;
        }
        let row = result?;
        //for i in 0..row_length {
        for (i, (column, value)) in zip(&columns, &row).enumerate() {
            //let column = &headers[i];
            //let value = &row[i];
            if infer {
                if let Some(sqltype) = infer::infer(value, sqltypes[i].index, sqltypes[i].subindex)
                {
                    sqltypes[i].merge(&sqltype);
                }
            }
            if i != 0 {
                write!(stream, "{}", field_sep)?;
            }
            field_processor(&mut stream, &column, &value)?;
        }
        if let Some(page_footer) = page_footer {
            if ((rounds + 1) % (page_size)) == 0 {
                page_footer(&mut stream, tablename, &columns, &sqltypes)?;
            }
        }
    }
    if let Some(page_footer) = page_footer {
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

fn schema_string(headers: &Vec<String>, sqltypes: &Vec<infer::SQLType>) -> String {
    let row_length = headers.len();
    let mut schema = String::new();
    for i in 0..row_length {
        let column = &headers[i];
        let sqlt = &sqltypes[i];
        schema.push_str(&format!("{} {}", column, sqlt));
        if i < row_length - 1 {
            schema.push_str(", ");
        }
    }
    schema
}

pub fn csv_to_bcp(csvfile: &PathBuf, bcpfile: &PathBuf) -> Result<()> {
    let mut rdr = csv_reader(csvfile)?;
    let outfile = File::create(bcpfile)?;
    let mut stream = BufWriter::new(outfile);
    let mut first_line = true;
    for result in rdr.records() {
        if !first_line {
            write!(stream, "\x1E")?;
        } else {
            first_line = false;
        }
        let row = result?;
        let fields = row.iter().collect::<Vec<&str>>();
        let line = fields.join("\x1F");
        write!(stream, "{}", line)?;
    }
    stream.flush()?;
    Ok(())
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nonexistent_csv_file() {
        let attempt = csv_reader(&PathBuf::from("No_Such_File.csv"));
        assert!(attempt.is_err());
    }
}
