use clap::Parser;
use makefile2doc::process;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'i', long, default_value = "Makefile")]
    input: PathBuf,

    #[arg(short = 'o', long)]
    output: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let content = match fs::read_to_string(&args.input) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: Unable to read the file '{}'", args.input.display());
            eprintln!("Details: '{}'", e);
            process::exit(1);
        }
    };

    let markdown = process(&content);

    let output_path = match args.output {
        Some(path) => path,
        None => {
            let parent = args.input.parent().unwrap_or(Path::new("."));
            parent.join("MAKEFILE.md")
        }
    };

    if let Err(e) = fs::write(&output_path, markdown) {
        eprintln!(
            "Error: Unable to write the file '{}'",
            output_path.display()
        );

        eprintln!("Details: {}", e);
        process::exit(1);
    }

    println!("Successfully generated documentation at {}", output_path.display());
}
