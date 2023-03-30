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
    // rdr.set_headers(new_headers);
    Ok(rdr)
}

// TODO: de-duplicate column names when needed
pub fn csv_columns(csvfile: &PathBuf, tablename: Option<String>, raw: bool) -> Result<Vec<String>> {
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

pub fn csv_schema(csvfile: &PathBuf, tablename: &str) -> Result<()> {
    let headers = csv_columns(csvfile, Some(tablename.to_string()), false)?;
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
    println!(
        "DROP TABLE IF EXISTS {0};\nCREATE TABLE {0} ({1});",
        tablename, schema
    );
    Ok(())
}

pub fn csv_into_json(
    csvfile: &PathBuf,
    filename: &PathBuf,
    tablename: &str,
    page_size: Option<usize>,
) -> Result<()> {
    let headers = csv_columns(csvfile, Some(tablename.to_string()), false)?;
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

pub fn csv_into(
    csvfile: &PathBuf,
    filename: &PathBuf,
    tablename: &str,
    infer: bool,
    json: bool,
    page_size: Option<usize>,
) -> Result<()> {
    let headers = csv_columns(csvfile, Some(tablename.to_string()), false)?;
    let row_sep: String;
    let mut json_row: Map<String, Value> = Map::new();
    let json_page_size: usize;
    if let Some(page_size) = page_size {
        json_page_size = page_size
    } else {
        json_page_size = 5000;
    }
    if json {
        row_sep = ", \\\n".to_string();
    } else {
        row_sep = "\x1E".to_string();
    }
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
            if json {
                writeln!(
                    stream,
                    "INSERT INTO {}\nSELECT * FROM OPENJSON('[ \\",
                    tablename
                )?;
            }
            first_line = false;
        }
        let row = result?;
        if json {
            json_row.clear();
        }
        //for i in 0..row_length {
        for (i, (column, value)) in zip(&headers, &row).enumerate() {
            //let column = &headers[i];
            //let value = &row[i];
            if infer {
                if let Some(sqltype) = infer::infer(value, sqltypes[i].index, sqltypes[i].subindex) {
                    sqltypes[i].merge(&sqltype);
                }
            }
            if json {
                json_row.insert(column.into(), value.into());
            }
        }
        if json {
            let json_text = serde_json::to_string(&json_row)?.replace("'", "''");
            //let fields = row.iter().collect::<Vec<&str>>();

            write!(stream, "{}", json_text)?;
            if ((row_num + 1) % (json_page_size)) == 0 {
                writeln!(
                    stream,
                    " \\\n]') WITH ({});\n",
                    schema_string(&headers, &sqltypes)
                )?;
                first_line = true;
            }
        }
    }
    let schema = schema_string(&headers, &sqltypes);
    if json && !first_line {
        writeln!(stream, " \\\n]') WITH ({});", schema)?;
    }

    stream.flush()?;
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
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
