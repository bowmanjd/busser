// Copyright 2023 Jonathan Bowman
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use anyhow::{Context, Result};
use csv::{Reader, ReaderBuilder, Terminator};
use std::fs::File;
use std::path::PathBuf;

pub struct InputConfig<'a> {
    pub file: &'a PathBuf,
    pub row_sep: Option<Vec<u8>>,
    pub field_sep: Option<Vec<u8>>,
    pub terminator: Option<Terminator>,
    pub delimiter: Option<u8>,
}

impl InputConfig<'_> {
    fn new(file: &PathBuf) -> InputConfig {
        InputConfig {
            file,
            row_sep: None,
            field_sep: None,
            terminator: None,
            delimiter: None,
        }
    }

    fn row_sep<'a>(&mut self) -> &'a [u8] {
        if let Some(row_sep) = self.row_sep {
            &row_sep
        } else {
            let row_sep = b"\n".to_vec();
            self.row_sep = Some(row_sep);
            &row_sep
        }
    }

    fn set_terminator<'a>(&mut self, sep: u8) {
        self.terminator = Some(Terminator::Any(sep));
    }

    fn terminator(&mut self) -> Terminator {
        if let Some(terminator) = self.terminator {
            terminator
        } else {
            let terminator = Terminator::CRLF;
            self.terminator = Some(terminator);
            terminator
        }
    }

    fn field_sep(&mut self) -> Vec<u8> {
        //self.field_sep.unwrap_or_else(|| b",".to_vec())
        if let Some(field_sep) = &self.field_sep {
            field_sep
        } else {
            let field_sep = b",";
            self.field_sep = Some(field_sep.to_vec());
            field_sep.to_vec()
        }
    }

    fn delimiter(&mut self) -> u8 {
        if let Some(delimiter) = self.delimiter {
            delimiter
        } else {
            let delimiter = b',';
            self.delimiter = Some(delimiter);
            delimiter
        }
    }
}

fn csv_reader(config: &mut InputConfig) -> Result<Reader<File>> {
    let rdr = ReaderBuilder::new()
        .delimiter(config.delimiter())
        .terminator(config.terminator())
        .buffer_capacity(65536)
        .from_path(config.file)
        .with_context(|| format!("Failed to read csv from {:?}", config.file))?;
    Ok(rdr)
}
