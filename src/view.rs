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

trait Displayable {
    fn display(&self) -> &str;
}

impl Displayable for str {
    fn display(&self) -> &str {
        self
    }
}

impl Displayable for &str {
    fn display(&self) -> &str {
        *self
    }
}

impl Displayable for String {
    fn display(&self) -> &str {
        self.as_str()
    }
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
    T::Output: Displayable,
{
    let pad = pad.unwrap_or(' ');
    print!("{}{}", left, pad);
    for (i, length) in lengths.iter().enumerate() {
        if i > 0 {
            print!("{0}{1}{0}", pad, inner);
        }
        if let Some(values) = values {
            print!("{:width$}", values[i].display(), width = length);
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

    let stats = crate::csv_survey(csvfile, false, None, field_sep, row_sep)?;
    let mut rdr = crate::csv_reader(csvfile, field_sep, row_sep)?;

    borders::<Vec<&str>>(
        None,
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

    borders::<Vec<&str>>(
        None,
        &stats.column_char_lengths,
        '\u{251C}',
        '\u{253C}',
        '\u{2524}',
        Some('\u{2500}'),
    );

    for result in rdr.records() {
        let row = result?;
        borders(
            Some(&row),
            &stats.column_char_lengths,
            '\u{2502}',
            '\u{2502}',
            '\u{2502}',
            None,
        );
    }

    borders::<Vec<&str>>(
        None,
        &stats.column_char_lengths,
        '\u{2514}',
        '\u{2534}',
        '\u{2518}',
        Some('\u{2500}'),
    );

    Ok(())
}
