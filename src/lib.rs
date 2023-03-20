use anyhow::{Context, Result};
use csv::{Reader, ReaderBuilder, StringRecord};
use serde_json::Map;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

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

    write!(stream, "[\n")?;

    for result in rdr.records() {
        if first_line == false {
            write!(stream, ",\n")?;
        } else {
            first_line = false;
        }
        let row = result?;
        let mut json_row = Map::new();
        let row_length: usize = row.len();
        for i in 0..row_length {
            let column = &headers[i];
            let value = &row[i];
            json_row.insert(column.into(), value.into());
        }
        let json_text = serde_json::to_string(&json_row)?;
        write!(stream, "{}", json_text)?;
    }
    write!(stream, "\n]")?;
    stream.flush()?;
    Ok(())
}

pub fn csv_to_bcp(csvfile: &PathBuf, bcpfile: &PathBuf) -> Result<()> {
    let mut rdr = csv_reader(csvfile)?;
    let outfile = File::create(bcpfile)?;
    let mut stream = BufWriter::new(outfile);
    let mut first_line = true;
    for result in rdr.records() {
        if first_line == false {
            write!(stream, "\x1E")?;
        } else {
            first_line = false;
        }
        let row = result?;
        let line = row.iter().collect::<Vec<&str>>().join("\x1F");
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
