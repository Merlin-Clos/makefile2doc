use clap::Parser;
use makefile2doc::process;
use std::fs;
use std::path::PathBuf;
use std::process;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(default_value = "Makefile")]
    input: PathBuf,

    #[arg(default_value = "MAKEFILE.md")]
    output: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let content = match fs::read_to_string(&args.input) {
        Ok(c) => c,
        Err(e) => {
            eprintln!(
                "Error: Impossible to read the file '{}'",
                args.input.display()
            );
            eprintln!("Details: '{}'", e);
            process::exit(1);
        }
    };

    let markdown = process(&content);

    match args.output {
        Some(path) => {
            if let Err(e) = fs::write(&path, markdown) {
                eprintln!(
                    "Error: Impossible to write the documentation '{}'",
                    path.display()
                );
                eprintln!("Details: '{}'", e);
                process::exit(1);
            }
        }
        None => {
            if let Err(e) = fs::write(&args.input, markdown) {
                eprintln!(
                    "Error: Impossible to write the documentation '{}'",
                    &args.input.display()
                );
                eprintln!("Details: '{}'", e);
                process::exit(1);
            }
        }
    }
}
