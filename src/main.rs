use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: PathBuf,
    /// Case insensitive matching
    #[arg(short, long)]
    ignore_case: bool,
}

fn main() -> Result<()> {
    let Cli {
        pattern,
        path,
        ignore_case,
    } = Cli::parse();

    let file = open_file(&path)?;

    find_matches(&file, &pattern, ignore_case)
}

fn open_file(path: &PathBuf) -> Result<File> {
    File::open(path).with_context(|| format!("Could not read file `{}`", path.to_str().unwrap()))
}

fn find_matches(file: &File, pattern: &str, ignore_case: bool) -> Result<()> {
    let reader = BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        let line = line.with_context(|| format!("Could not read line `{}`", index))?;

        if (ignore_case && line.to_lowercase().contains(&pattern.to_lowercase()))
            || line.contains(pattern)
        {
            // let line = curr.replace(pattern, pattern.red().bold().to_string().as_str());
            let line_num = (index + 1).to_string().dimmed();
            println!("{: >3} {}", line_num, line);
        }
    }
    Ok(())
}
