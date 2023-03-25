use anyhow::{Context, Result};
use csv::{Reader, ReaderBuilder, StringRecord};
use serde_json::Map;
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

pub fn csv_to_json(csvfile: &PathBuf, jsonfile: &PathBuf) -> Result<()> {
    let mut rdr = csv_reader(csvfile)?;
    let outfile = File::create(jsonfile)?;
    let mut stream = BufWriter::new(outfile);
    let headers = rdr.headers()?.clone();
    let mut first_line = true;
    let row_length: usize = headers.len();
    writeln!(stream, "[")?;
    //let mut sqltypes: Vec<infer::SQLType> = Vec::with_capacity(row_length);
    let mut sqltypes: Vec<infer::SQLType> = vec![infer::SQLType { ..Default::default() }; row_length];

    for result in rdr.records() {
        if !first_line {
            writeln!(stream, ",")?;
        } else {
            first_line = false;
        }
        let row = result?;
        let mut json_row = Map::new();
        for i in 0..row_length {
            let column = &headers[i];
            let value = &row[i];
            if let Some(sqltype) = infer::infer(value, sqltypes[i].index) {
                if sqltype.fixed && sqltypes[i].fixed && sqltype.size != sqltypes[i].size {
                    if let Some(unfixed) = infer::infer(value, sqltypes[i].index + 1) {
                        sqltypes[i] = unfixed;
                    }
                }
                if sqltype.index > sqltypes[i].index {
                    sqltypes[i] = sqltype;
                } else if sqltype.index == sqltypes[i].index {
                    if sqltype.size > sqltypes[i].size {
                        sqltypes[i].size = sqltype.size;
                    }
                    if sqltype.scale > sqltypes[i].scale {
                        sqltypes[i].scale = sqltype.scale 
                    }
                }
            }
            json_row.insert(column.into(), value.into());
        }
        let json_text = serde_json::to_string(&json_row)?;
        //let fields = row.iter().collect::<Vec<&str>>();
        write!(stream, "{}", json_text)?;
    }
    write!(stream, "\n]")?;
    println!("{:?}", sqltypes);
    stream.flush()?;
    Ok(())
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
        println!("{:?}", fields);
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
