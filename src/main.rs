use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let Cli { pattern, path } = Cli::parse();

    let file = File::open(&path).with_context(|| {
        format!(
            "could not read file `{}`",
            &path.into_os_string().into_string().unwrap()
        )
    })?;
    let mut reader = BufReader::new(file);

    find_matches(&mut reader, &pattern)
}

fn find_matches(content: &mut BufReader<File>, pattern: &str) -> Result<()> {
    for (index, line) in content.lines().enumerate() {
        let curr = line?;
        if curr.contains(pattern) {
            println!("line {} - {}", index, curr.trim());
        }
    }
    Ok(())
}
