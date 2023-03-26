use anyhow::{Context, Result};
use csv::{Reader, ReaderBuilder, StringRecord};
use serde_json::{Map, Value};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub mod infer;

fn csv_reader(csvfile: &PathBuf) -> Result<Reader<File>> {
    let mut rdr = ReaderBuilder::new()
        .from_path(csvfile)
        .with_context(|| format!("Failed to read csv from {:?}", csvfile))?;
    let headers = rdr.headers()?;
    let new_headers: StringRecord = headers
        .iter()
        .map(|h| {
            h.chars()
                .map(|c| if c.is_alphanumeric() { c } else { '_' })
                .collect::<String>()
        })
        .collect();
    rdr.set_headers(new_headers);
    Ok(rdr)
}

pub fn read_csv_headers(csvfile: &PathBuf) -> Result<Vec<String>> {
    let mut rdr = csv_reader(csvfile)?;
    let headers = rdr.headers()?;
    Ok(headers.iter().map(str::to_string).collect())
}

pub fn csv_into(
    csvfile: &PathBuf,
    filename: &PathBuf,
    tablename: &str,
    infer: bool,
    json: bool,
    page_size: Option<usize>,
) -> Result<()> {
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
    let headers = rdr.headers()?.clone();
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
        for i in 0..row_length {
            let column = &headers[i];
            let value = &row[i];
            if infer {
                if let Some(sqltype) = infer::infer(value, sqltypes[i].index) {
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

fn schema_string(headers: &StringRecord, sqltypes: &Vec<infer::SQLType>) -> String {
    let row_length = headers.len();
    let mut schema = String::new();
    for i in 0..row_length {
        let column = &headers[i];
        let sqlt = &sqltypes[i];
        schema.push_str(&format!("[{}] {}", column, sqlt));
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
