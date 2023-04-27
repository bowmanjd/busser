use anyhow::Result;
use std::path::PathBuf;

pub fn range_from_string(range_str: &str) -> Result<Vec<Vec<usize>>> {
    Ok(range_str
        .split(',')
        .map(|x| x.trim())
        .map(|x| {
            x.split('-')
                .map(|x| x.parse::<usize>().unwrap_or(0))
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>())
}

fn borders<T>(
    values: Option<&T>,
    lengths: &[usize],
    left: char,
    inner: char,
    right: char,
    pad: Option<char>,
) where
    T: std::ops::Index<usize>,
    T::Output: AsRef<str>,
{
    let pad = pad.unwrap_or(' ');
    print!("{}{}", left, pad);
    for (i, length) in lengths.iter().enumerate() {
        if i > 0 {
            print!("{0}{1}{0}", pad, inner);
        }
        if let Some(values) = values {
            print!("{:width$}", values[i].as_ref(), width = length);
        } else {
            print!("{}", pad.to_string().repeat(*length));
        };
    }
    println!("{}{}", pad, right);
}

pub fn view(
    csvfile: &PathBuf,
    rows: Option<&str>,
    columns: Option<&str>,
    field_sep: Option<u8>,
    row_sep: Option<u8>,
    ascii_delimited: bool,
    numbered: bool,
) -> Result<()> {
    let field_sep = if ascii_delimited && field_sep.is_none() {
        Some(b'\x1F')
    } else {
        field_sep
    };
    let row_sep = if ascii_delimited && row_sep.is_none() {
        Some(b'\x1E')
    } else {
        row_sep
    };

    let row_range = if let Some(rows) = rows {
        range_from_string(rows)?
    } else {
        vec![vec![0]]
    };

    let col_range = if let Some(columns) = columns {
        range_from_string(columns)?
    } else {
        vec![vec![0]]
    };
    println!("row_range: {:?}\ncol_range: {:?}", row_range, col_range);

    let mut stats = crate::csv_survey(csvfile, false, None, field_sep, row_sep)?;
    let mut rdr = crate::csv_reader(csvfile, field_sep, row_sep)?;

    //let mut column_lengths = &mut stats.column_char_lengths;

    if numbered {
        let row_num_length = stats.row_count.to_string().len(); 
        //let col_num_length = stats.column_count.to_string().len();
        for i in 0..stats.column_count {
            stats.raw_columns[i] = format!("{}: {}", i + 1, stats.raw_columns[i]);
            stats.column_char_lengths[i] = stats.column_char_lengths[i].max(stats.raw_columns[i].len());
        }
        stats.column_char_lengths.insert(0, row_num_length);
        stats.raw_columns.insert(0, "#".to_string());
    }

    let nothing: Option<&Vec<&str>> = None;

    borders(
        nothing,
        &stats.column_char_lengths,
        '\u{250C}',
        '\u{252c}',
        '\u{2510}',
        Some('\u{2500}'),
    );

    borders(
        Some(&stats.raw_columns),
        &stats.column_char_lengths,
        '\u{2502}',
        '\u{2502}',
        '\u{2502}',
        None,
    );

    borders(
        nothing,
        &stats.column_char_lengths,
        '\u{251C}',
        '\u{253C}',
        '\u{2524}',
        Some('\u{2500}'),
    );

    let mut last_row: usize = 0;

    for range in row_range {
        let first_row = if range[0] == 0 {
            1
        } else {
            range[0]
        };
        //let skip = first_row + last_row;
        let skip = range[0].saturating_sub(1 + last_row);
        last_row = range[range.len() - 1];
        if last_row == 0 {
            last_row = stats.row_count;
        }
        let rows = last_row - skip;
        for (i, result) in rdr.records().skip(skip).take(rows).enumerate() {
            let mut row: Vec<String> = result?.iter().map(str::to_owned).collect();
            if numbered {
                row.insert(0, (i + first_row).to_string());
            }
            borders(
                Some(&row),
                &stats.column_char_lengths,
                '\u{2502}',
                '\u{2502}',
                '\u{2502}',
                None,
            );
        }
    }

    borders(
        nothing,
        &stats.column_char_lengths,
        '\u{2514}',
        '\u{2534}',
        '\u{2518}',
        Some('\u{2500}'),
    );

    Ok(())
}
