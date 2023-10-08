use clap::Parser;
use std::time::Instant;

#[derive(Parser)]
#[command(author, version, about = "Grep inside a file", long_about = None)]
struct  Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let start = Instant::now();
    let args = Cli::parse();

    let result = std::fs::read_to_string(args.path);
    match result {
        Ok(content) => { grep(content, args.pattern) }
        Err(error) => { eprintln!("Oops: {}", error) }
    }

    println!("Took {:?}", start.elapsed());
}

fn grep(content: String, pattern: String) {
    for line in content.lines() {
        if line.contains(&pattern) {
            println!("{}", line);
        }
    }
}