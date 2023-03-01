use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: PathBuf,
}

fn main() -> Result<()> {
    let Cli { pattern, path } = Cli::parse();

    let file = open_file(&path)?;

    find_matches(&file, &pattern)
}

fn open_file(path: &PathBuf) -> Result<File> {
    File::open(path).with_context(|| format!("could not read file `{}`", path.to_str().unwrap()))
}

fn find_matches(file: &File, pattern: &str) -> Result<()> {
    let reader = BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        let curr = line?;
        if curr.contains(pattern) {
            println!("line {} - {}", index + 1, curr.trim());
        }
    }
    Ok(())
}
