use std::env;
use std::fs;
use std::process;
use std::error::Error;
use make_docs::search;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
        
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

struct Config {
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments"); // Je sais pas pourquoi c'est < 2 car techniquement je lance qu'avec 1 argument ? cargo run -- Makefile, donc si je fais cargo run -- ça devrait être 0 argument nan ?
        }
        let file_path = args[1].clone();
        
        if file_path != "Makefile" {
            return Err("makedoc only make the documentation for Makefiles");
        }
        
        Ok(Config { file_path })
    }
}

fn run(config: Config) -> Result <(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
       
    for line in search(&contents) {
        println!("{line}");
    }
    
    println!("With text:\n{contents}");
    
    Ok(())
}