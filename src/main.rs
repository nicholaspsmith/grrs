use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let Cli { pattern, path } = args;
    let content = std::fs::read_to_string(&path).expect("could not read file");

    for line in content.lines() {
        if line.contains(&pattern) {
            println!("{}", line);
        }
    }
}
