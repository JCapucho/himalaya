use anyhow::Result;
use std::io;
use termcolor::{self, StandardStream};

use crate::config::Format;

pub trait WriteColor: io::Write + termcolor::WriteColor {}

impl WriteColor for StandardStream {}

pub trait PrintTable {
    fn print_table(&self, writer: &mut dyn WriteColor, opts: PrintTableOpts) -> Result<()>;
}

pub struct PrintTableOpts<'a> {
    pub format: &'a Format,
    pub max_width: Option<usize>,
}
