use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let Cli { pattern, path } = args;

    let file_path = Path::new(&path);

    // open file
    let file =
        File::open(file_path).with_context(|| format!("Could not read file `{:?}`", file_path))?;

    let reader = BufReader::new(file);

    // iterate over lines
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(content) => content,
            Err(error) => {
                return Err(error.into());
            }
        };
        if line.contains(&pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
