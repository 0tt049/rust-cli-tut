#![allow(unused)]

use anyhow::{Context, Result};

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, Write};
use std::path::PathBuf;

use clap::Parser;

use ansi_term::Colour::Green;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());

    let patt = &args.pattern.to_string();
    let path = &args.path;
    let content = File::open(path).with_context(|| format!("could not read file `{:?}`", path))?;
    let mut buf = BufReader::new(content);
    let mut i = 0;
    buf.lines().for_each(|f| {
        i += 1;
        let mut my_f = f.as_ref().unwrap();
        if my_f.contains(patt) {
            writeln!(handle, "Line {} -- {}", i, Green.bold().paint(my_f.to_string()));
        }
    });
    Ok(())
}
