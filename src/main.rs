use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
struct CustomError(String);

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<(), CustomError> {
    let args = Cli::parse();
    let Cli { pattern, path } = args;

    let file_path = Path::new(&path);

    // open file
    let file = File::open(file_path)
        .map_err(|err| CustomError(format!("Error reading `{:?}`: {}", file_path, err)))?;

    let reader = BufReader::new(file);

    // iterate over lines
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(content) => content,
            Err(error) => {
                return Err(CustomError(format!(
                    "Error reading file: {:?}",
                    error.to_string()
                )));
            }
        };
        if line.contains(&pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
